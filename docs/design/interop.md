# Interoperability and no vendor lock-in

Aerogram is part of the Nubster product family, but it does not require any other Nubster brick to run. Every external contract is expressed through an open standard. The Nubster bricks listed in [`architecture.md`](architecture.md) are convenient defaults, never hard dependencies.

## The no-lock-in test

A self-hosted operator must be able to run Aerogram end to end against an entirely non-Nubster stack:

```
Aerogram + <your OIDC provider> + PostgreSQL + RabbitMQ
```

This combination is exercised by the example under [`examples/02-self-hosted-oidc/`](../../examples/02-self-hosted-oidc/). If a future change ever requires a Nubster-specific component on this path, the change is rejected.

## Contracts at a glance

| Concern | Open standard | Default Nubster integration | Alternative |
| --- | --- | --- | --- |
| Admin authentication | OIDC (RFC 6749 / OpenID Connect Core 1.0) + JWKS discovery | Nubster Identity | Any standards-compliant OIDC provider |
| API authorisation | Bearer JWT signed by the configured IdP | Tokens minted by Nubster Identity | Tokens minted by any IdP whose JWKS is reachable |
| Tenant provisioning | SCIM 2.0 (RFC 7643 / 7644) endpoint exposed by Aerogram | Nubster Identity SCIM client | Any SCIM 2.0-compliant connector |
| Multi-tenant identifier | `tenant_id` UUID opaque, claimed from the JWT | Subject ID of Nubster Identity | Any claim configured in `auth.tenant_claim` |
| Outgoing events | CloudEvents v1.0 JSON | Optional Hexeract Outbox adapter (feature flag) | Webhook HMAC / RabbitMQ / NATS / Kafka / any HTTP receiver |
| Webhook signature | HMAC-SHA-256 over the raw body | Same signature on all targets | Same |
| Dev orchestration | `lightshuttle.yml` manifest checked into the repository | LightShuttle | `docker-compose.yml` (also provided as a fallback) |
| Reverse proxy | HTTP/1.1 and HTTP/2 standard | Nubster Platform gateway | Caddy / nginx / Traefik / HAProxy |
| Object storage | S3-compatible API (`PutObject`, `GetObject`, multipart) | None | MinIO / Backblaze B2 / Cloudflare R2 / AWS S3 |
| DNS resolution | RFC 1035 + DNS over TCP/UDP | None | Any DNS server, including the host system resolver |

## OIDC authentication

`aerogram-auth` is a generic OIDC client. The configuration block lives under `[auth.oidc]`:

```toml
[auth.oidc]
issuer = "https://id.example.org"
client_id = "aerogram-admin"
client_secret = "${env.AEROGRAM_OIDC_CLIENT_SECRET}"
redirect_uri = "https://mail.example.org/admin/callback"
jwks_cache_ttl = "1h"
tenant_claim = "tenant_id"   # any custom claim; default if absent
```

No code under `aerogram-auth` references Nubster Identity directly. The crate consumes the OIDC discovery document (`/.well-known/openid-configuration`) and the JWKS published by the provider. Switching providers is a configuration change, not a code change.

## SCIM 2.0 provisioning

Aerogram exposes a SCIM 2.0 endpoint at `/scim/v2/` so any compliant provisioning source can create, update and delete mailboxes:

- `POST /scim/v2/Users` provisions a mailbox.
- `PATCH /scim/v2/Users/{id}` updates the mailbox (forwarding address, quota, status).
- `DELETE /scim/v2/Users/{id}` deactivates the mailbox.

The endpoint is authenticated through the same OIDC bearer token mechanism, with a dedicated scope `scim:write` documented in `spec/api-v1.md`.

## CloudEvents outgoing events

Aerogram emits the following events for every outbound message:

- `mail.delivered`
- `mail.bounced`
- `mail.complained`
- `mail.opened`
- `mail.clicked`

Each event is a CloudEvents v1.0 envelope. The default sink is the operator-configured webhook URL. Additional sinks are activated by feature flags:

- `hexeract` on `aerogram-events`: pushes the same event into a Hexeract Outbox table.
- `rabbitmq` (v0.2+): publishes the same event to a configured RabbitMQ exchange.
- `kafka` (v0.2+): publishes the same event to a configured Kafka topic.

The CloudEvents JSON schema is published under `spec/cloudevents/` and is part of the public API per [SEMVER_POLICY.md](../SEMVER_POLICY.md).

## Webhook HMAC signature

Webhook payloads are signed using HMAC-SHA-256 over the raw HTTP body. The signature travels in the `X-Aerogram-Signature` header in the form `t=<timestamp>,v1=<hex-signature>`. The verification snippet is documented in `spec/api-v1.md` and reproduced in every example under `examples/`.

## Mailbox model

The `Mailbox` entity is autonomous: `(tenant_id UUID, address String)` is the primary key. There is no foreign key to any external identity store. The optional view that links a mailbox to a Nubster Identity Account lives on the Identity side, not in Aerogram. See [`mailbox-model.md`](mailbox-model.md) for the detail.

## Storage and infrastructure

- The metadata schema is portable PostgreSQL 16+. No proprietary extension required.
- The blob backend defaults to the local filesystem with a layered path under `${store.blob_dir}/<tenant_id>/<yyyy>/<mm>/<dd>/<blob_id>`. The `s3` feature switches to any S3-compatible bucket through the AWS SDK.
- The full-text search index uses tantivy, an embedded Rust library. No Elasticsearch, no OpenSearch.
- DNS resolution uses hickory-resolver in async mode. The operator can override the resolver list with `[dns] servers = [...]`.

## What this means for contributors

Any pull request that introduces a hard dependency on a Nubster brick (a SDK import, a custom RPC, a schema requiring a Nubster table) will be rejected. The conversation reopens only if the contract is expressible as an open standard with a non-Nubster reference implementation.
