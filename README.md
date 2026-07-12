# Aerogram

> Self-hosted Rust email server: SMTP MTA, JMAP, IMAP, transactional API, DKIM/SPF/DMARC, multi-tenant, with no vendor lock-in.

[![crates.io](https://img.shields.io/crates/v/aerogram.svg?label=crates.io)](https://crates.io/crates/aerogram)
[![docs.rs](https://img.shields.io/docsrs/aerogram?label=docs.rs)](https://docs.rs/aerogram)
[![CI](https://github.com/nubster-opensources/aerogram/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/nubster-opensources/aerogram/actions/workflows/ci.yml)
[![MSRV](https://img.shields.io/badge/MSRV-1.88-blue.svg)](./docs/MSRV_POLICY.md)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](#license)
[![Status](https://img.shields.io/badge/status-pre--alpha-orange)](#status)
[![Made with Rust](https://img.shields.io/badge/made%20with-Rust-orange?logo=rust)](https://www.rust-lang.org/)

Aerogram is a server-side mail platform written in Rust. It bundles a sending and receiving SMTP server, a JMAP and IMAP store, a transactional REST API, native DKIM/SPF/DMARC, a Rust-native antispam engine and a self-hosted admin and webmail into a single binary. The design favours self-hostability: one process, one configuration file, one container image. Every external integration goes through an open standard so the operator can keep their existing identity provider, message bus and observability stack.

Aerogram is sponsored by [Nubster](https://nubster.com).

## Status

🚧 **Pre-alpha, no usable release yet.**

The repository is intentionally public from day one to capture the name and make the design discussion visible. **Do not depend on it yet**, anything can change until v0.1.0.

| Feature | v0.1.0 |
| --- | --- |
| Outbound SMTP and DKIM signing (Ed25519, RSA-SHA256) | ⏳ M1 |
| Transactional API (`POST /v1/messages`, webhooks HMAC-SHA-256) | ⏳ M1 - M2 |
| CloudEvents emission with optional Hexeract Outbox adapter | ⏳ M2 |
| Inbound SMTP with STARTTLS, DKIM/SPF/DMARC verification | ⏳ M3 |
| Metadata store (PostgreSQL) and blob store (filesystem default, S3-compatible) | ⏳ M4 |
| JMAP server (RFC 8620 + 8621) | ⏳ M5 |
| IMAP4rev2 (RFC 9051) compat shim | ⏳ M6 |
| Rust-native antispam (Bayesian, DNS blocklists, greylisting) | ⏳ M7 |
| OIDC client and SCIM 2.0 provisioning endpoint | ⏳ M8 |
| SSR webmail and admin dashboard (Askama + HTMX) | ⏳ M9 |
| Release polish, security audit and fuzzing | ⏳ M10 |

See the [roadmap](./docs/explanation/roadmap.md) for the detailed trajectory and the [CHANGELOG](./CHANGELOG.md) for the published history.

## Quick start

> Pre-alpha. Nothing below works end to end yet. This section illustrates the targeted experience for v0.1.0.

**Prerequisites.** A Linux host with ports 25, 465, 587, 993, 4190, 8080 reachable, a PostgreSQL 16+ instance and a domain you control for DKIM and DMARC setup.

```bash
# Generate a default configuration and DKIM keypair
aerogram init --domain example.org --postgres-url postgres://aerogram@localhost/aerogram

# Apply database migrations
aerogram migrate

# Start the server
aerogram start --config /etc/aerogram/aerogram.toml
```

Send a transactional message through the API:

```bash
curl -X POST https://mail.example.org/v1/messages \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "from": "noreply@example.org",
    "to": ["alice@example.com"],
    "subject": "Hello",
    "text": "Sent through Aerogram."
  }'
```

## Why Aerogram

Self-hosting transactional mail today means stitching together Postfix, OpenDKIM, Dovecot, Rspamd, custom delivery dashboards and a third-party transactional provider. The result is hard to operate, hard to upgrade and hard to reason about under regulatory pressure. Aerogram closes that gap with a single binary that covers the full surface area while keeping the operator in control:

- **One process.** SMTP in, SMTP out, JMAP, IMAP, API, admin, webmail, all in the same `aerogram` binary.
- **One configuration file.** TOML, validated at start-up, hot-reloadable.
- **Open standards everywhere.** OIDC for admin auth, SCIM for provisioning, CloudEvents for outgoing events, HMAC for webhooks.
- **No vendor lock-in.** The default integrations point at Nubster Identity and Hexeract because they are the convenient option in the Nubster product family, but every contract is an open standard. Keep your existing OIDC provider, RabbitMQ and PostgreSQL setup as is.
- **Rust-native antispam.** No Lua bindings, no plugin loader, no second daemon to keep in sync.
- **Multi-tenant by design.** A single instance handles many domains and many tenants with isolation enforced at the store, queue and search-index layers.

The bet behind Aerogram is that a focused Rust codebase with a sane configuration model is more maintainable than the layered traditional mail stack.

## What Aerogram is **not**

To stay focused, the following are explicitly out of scope:

- **Not a marketing automation platform.** Aerogram delivers messages; segmentation, campaigns and A/B testing belong elsewhere.
- **Not a CRM.** No contact deduplication, no opportunity tracking, no sales pipeline.
- **Not a calendaring server.** CalDAV and CardDAV may ship as a sibling brick of the Nubster product family, never inside Aerogram.
- **Not a curator of centralised reputation.** Aerogram applies the antispam policies you configure; it does not push a centrally maintained blocklist or override the operator's deliverability decisions.
- **Not an email archive engine.** Aerogram stores live mailboxes. Long-term legal-hold archival belongs in dedicated archival software.

## Audience

- **Self-hosted operators** running their own mail infrastructure and looking for a maintainable Rust alternative to a stack of C and Perl daemons.
- **Teams running their own infrastructure** who need a transactional API without relying on external mail providers.
- **Application teams** dogfooding the Nubster product family and looking to retire their dependency on a third-party transactional mail provider without giving up on JMAP and IMAP for their human users.

## Documentation

- [docs/explanation/roadmap.md](./docs/explanation/roadmap.md) - milestone-by-milestone trajectory to v1.0
- [CHANGELOG.md](./CHANGELOG.md) - published release history
- [docs/SEMVER_POLICY.md](./docs/SEMVER_POLICY.md) - versioning guarantees
- [docs/MSRV_POLICY.md](./docs/MSRV_POLICY.md) - minimum supported Rust version policy

## Contributing

Contributions are welcome. Please read [`CONTRIBUTING.md`](./CONTRIBUTING.md) first for the workflow and conventions, and [`CODE_OF_CONDUCT.md`](./CODE_OF_CONDUCT.md) for the community guidelines. For vulnerability reports, see [`SECURITY.md`](./SECURITY.md). For open-ended questions and design conversations, use [GitHub Discussions](https://github.com/nubster-opensources/aerogram/discussions).

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual-licensed as above, without any additional terms or conditions.

See [CONTRIBUTING.md](CONTRIBUTING.md) for details, including the Contributor License Agreement (CLA).

Copyright © Nubster.
