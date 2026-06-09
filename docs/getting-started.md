# Getting started

> Aerogram is pre-alpha. Nothing below works end to end yet. This page describes the targeted onboarding experience for v0.1.0.

## Prerequisites

- A Linux host with the standard mail ports reachable from the Internet: `25` (inbound SMTP), `465` (submission over TLS), `587` (submission with STARTTLS), `993` (IMAPS), `4190` (managesieve, optional, v0.3+), `8080` (admin and webmail).
- A PostgreSQL 16+ instance.
- A domain you control, with permission to set `TXT`, `MX` and `CNAME` records.
- A blob store: local filesystem by default, or any S3-compatible bucket (with the `s3` feature flag).
- Optional: an OIDC provider for admin authentication (Nubster Identity or any standards-compliant OIDC provider).

## Install

```bash
cargo install --locked aerogram
```

## Initialise

```bash
aerogram init \
  --domain example.org \
  --postgres-url postgres://aerogram@localhost/aerogram \
  --config-out /etc/aerogram/aerogram.toml
```

`aerogram init` generates a TOML configuration, an Ed25519 DKIM keypair, an RSA-SHA256 DKIM keypair, prints the DNS records you must publish (DKIM, SPF, DMARC, MTA-STS, TLS-RPT) and stores private material under the configured key directory with restrictive permissions.

## Migrate the database

```bash
aerogram migrate
```

This applies the SQL migrations under `aerogram-store`, `aerogram-queue`, `aerogram-spam` and `aerogram-auth` to the configured PostgreSQL instance. Idempotent; safe to re-run.

## Start the server

```bash
aerogram start --config /etc/aerogram/aerogram.toml
```

The single binary boots:

- the SMTP listeners (25, 465, 587),
- the IMAP listener (993, behind TLS),
- the JMAP and HTTP API listener (8080),
- the admin and webmail SSR endpoints under the same HTTP listener,
- the queue worker and the antispam scoring worker.

## Send a transactional message

```bash
curl -X POST https://mail.example.org/v1/messages \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "from": "noreply@example.org",
    "to": ["alice@example.com"],
    "subject": "Hello from Aerogram",
    "text": "It works."
  }'
```

The response carries a `message_id`. Subsequent delivery events (`delivered`, `bounced`, `complained`, `opened`, `clicked`) are pushed to the configured webhook endpoint with an HMAC-SHA-256 signature in the `X-Aerogram-Signature` header.

## Authenticate the admin through any OIDC provider

Aerogram acts as an OIDC client. Point it at any compliant provider in the configuration file:

```toml
[auth.oidc]
issuer = "https://id.example.org"
client_id = "aerogram-admin"
client_secret = "{{ from env }}"
redirect_uri = "https://mail.example.org/admin/callback"
```

The same flow works with Nubster Identity or any other standards-compliant OIDC provider. There is no Aerogram-specific SDK to install; the no-vendor-lock-in policy is documented in [`design/interop.md`](design/interop.md).

## Where to go next

- [`design/architecture.md`](design/architecture.md): an overview of the seventeen crates and the dataflow.
- [`design/interop.md`](design/interop.md): the open-standard contracts (OIDC, SCIM, CloudEvents, HMAC) that prevent lock-in.
- [`design/mailbox-model.md`](design/mailbox-model.md): the autonomous mailbox entity and the optional Identity view.
- [`spec/api-v1.md`](spec/api-v1.md): the transactional REST API surface.
