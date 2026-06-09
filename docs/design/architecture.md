# Architecture overview

Aerogram is a workspace of seventeen Rust crates that compile into a single `aerogram` binary. The crates are partitioned along functional boundaries: domain types, transport servers, validation, storage, search, protocol surfaces, integration points and the composition root.

## Layered view

```
                     ┌──────────────────────────────┐
                     │   aerogram (binary, CLI)     │
                     └──────────────┬───────────────┘
                                    │
                     ┌──────────────▼───────────────┐
                     │      aerogram-server         │ ◀── composition root
                     └─┬───────┬───────┬───────┬────┘
                       │       │       │       │
       ┌───────────────┘       │       │       └───────────────┐
       │                       │       │                       │
┌──────▼───────┐  ┌────────────▼───┐  ┌▼──────────────┐  ┌─────▼────────┐
│ aerogram-smtp│  │ aerogram-jmap  │  │ aerogram-imap │  │ aerogram-api │
│ (SMTP in/out)│  │ (RFC 8620/21) │  │  (RFC 9051)  │  │  (REST API)  │
└──┬───────┬───┘  └─────┬──────┬───┘  └─────┬─────────┘  └──┬─────┬────┘
   │       │            │      │            │               │     │
   │  ┌────▼────┐   ┌───▼──┐   │            │               │     │
   │  │ -dkim   │   │-spam │   │            │               │     │
   │  │ -spf-   │   │      │   │            │               │     │
   │  │  dmarc  │   └──────┘   │            │               │     │
   │  └─────────┘              │            │               │     │
   │                           │            │               │     │
   ▼                           ▼            ▼               ▼     ▼
┌──────────────┐         ┌──────────────┐              ┌──────────────────┐
│aerogram-queue│         │aerogram-store│              │aerogram-auth     │
│ (PG, retry)  │         │ (PG + blobs) │              │ (OIDC + SCIM)    │
└──────┬───────┘         └──────┬───────┘              └──────────────────┘
       │                        │
       │                  ┌─────▼────────┐
       │                  │aerogram-search│
       │                  │  (tantivy)   │
       │                  └──────────────┘
       │
┌──────▼─────────┐       ┌──────────────────┐       ┌──────────────────┐
│aerogram-events │       │ aerogram-admin   │       │ aerogram-config  │
│ (CloudEvents,  │       │ (Askama + HTMX)  │       │ (TOML + validate)│
│  webhooks,     │       │ admin + webmail  │       │                  │
│  Hexeract opt) │       └──────────────────┘       └──────────────────┘
└────────────────┘
                    ┌──────────────────────────────┐
                    │       aerogram-core           │
                    │  (TenantId, MailboxId,        │
                    │   EmailId, Address, Envelope, │
                    │   Header, MimeType, Error)    │
                    └──────────────────────────────┘
```

Every functional crate depends on `aerogram-core`. `aerogram-server` depends on every functional crate. The binary crate `aerogram` is a thin wrapper that parses the CLI, sets up logging, and hands control to `aerogram-server::Server`.

## Outbound dataflow

```
API request ──► aerogram-api validates, signs HMAC
              │
              ▼
              aerogram-queue persists a queue item with backoff policy
              │
              ▼
              aerogram-smtp client picks up the item
              ├─► aerogram-dkim signs the message (Ed25519 + RSA-SHA256)
              ├─► hickory-resolver looks up MX records
              ├─► attempts SMTP delivery with STARTTLS
              │     ├─► success ─► aerogram-events emits MailDelivered
              │     ├─► soft bounce ─► aerogram-queue re-queues with backoff
              │     └─► hard bounce ─► aerogram-events emits MailBounced,
              │                       suppression list updated
              ▼
              webhooks fired through aerogram-events,
              optionally relayed to Hexeract Outbox (feature flag)
```

## Inbound dataflow

```
Remote MTA ──► aerogram-smtp server (RFC 5321)
              ├─► STARTTLS handshake
              ├─► AUTH SASL (optional for trusted relays)
              ├─► aerogram-dkim verifies signature
              ├─► aerogram-spf-dmarc evaluates policy
              ├─► aerogram-spam scores message (Bayesian, DNSBL, greylist)
              ▼
              aerogram-store persists metadata + blob
              ├─► metadata in PostgreSQL (tenant_id, mailbox_id, ...)
              └─► blob in filesystem (default) or S3-compatible (feature s3)
                  │
                  ▼
                  aerogram-search indexes headers and body via tantivy
                  │
                  ▼
                  JMAP / IMAP clients read through aerogram-jmap / aerogram-imap
```

## Cross-cutting concerns

- **Authentication.** All admin and API surfaces validate JWTs through `aerogram-auth`, which fetches JWKS from any compliant OIDC provider. There is no hard dependency on Nubster Identity.
- **Configuration.** `aerogram-config` parses a single TOML file, validates cross-section invariants and exposes a hot-reload mechanism for non-structural settings.
- **Observability.** Every crate emits `tracing` spans; the binary configures an OpenTelemetry exporter through `aerogram-server`.
- **Events.** Outgoing domain events are serialised in CloudEvents v1.0 JSON. The default sink is the configured webhook URL; the `hexeract` feature on `aerogram-events` activates an optional Hexeract Outbox adapter.

## Process model

`aerogram start` boots a single Tokio runtime that spawns:

- one task per listener (SMTP 25, SMTP 465, SMTP 587, IMAPS 993, HTTP 8080),
- one queue worker per backend (outbound delivery, antispam scoring, webhook delivery),
- one OpenTelemetry exporter,
- one signal handler for graceful shutdown.

No daemon-per-feature split is required. The single binary owns the whole lifecycle.

## Mapping to the Nubster product family

| Nubster brick | Aerogram dependency |
| --- | --- |
| Nubster Identity | Optional OIDC provider for admin auth (any compliant IdP works). |
| Hexeract Outbox | Optional event sink via feature flag on `aerogram-events`. |
| Hexeract Bus | Out of scope for v0.1; CloudEvents sink can target any AMQP/MQTT broker. |
| LightShuttle | Used for local development; example manifest lives under `examples/`. |
| Nubster Platform | Optional gateway in front of the HTTP listener. Any reverse-proxy works. |
| MnemoDB / StyxDB / ThemisDB | Out of scope for v0.1; long-term audit can target StyxDB through CloudEvents. |

Each row above is a default convenient option, never a hard dependency. See [`interop.md`](interop.md) for the contracts in detail.
