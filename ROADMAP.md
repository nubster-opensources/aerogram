# Roadmap

Aerogram is pre-alpha. This document captures the intended trajectory of
the project up to v1.0, ordered by release. **No dates are committed.** The
project is sponsored on a best-effort basis by Nubster, and
releases ship when they are ready, not when a calendar says so.

The roadmap mirrors the GitHub milestones one-for-one. Each section here is
the public, prose form of a milestone; each milestone groups the issues that
must close before the release ships. The full design notes for any given
release live under `docs/design/` and `docs/spec/`.

## Out of scope

Aerogram is a complete mail server with a transactional API on top. The
following will never be in scope, regardless of demand:

- **Email marketing campaigns and segmentation.** Aerogram delivers messages; it does not run marketing automation.
- **CRM features.** No contact deduplication across organisations, no opportunity tracking, no funnel reporting.
- **Anti-abuse policing.** Aerogram applies the antispam and reputation policies you configure; it does not push a centrally curated blocklist or rate-limit reputational decisions for the operator.
- **Calendaring and contacts (CalDAV / CardDAV).** Out of scope; a separate Nubster brick may cover this in the future.
- **Webmail SPA in v0.1.x.** A minimal SSR webmail ships in v0.1, but a full-featured single-page webmail belongs in v0.3+.

These boundaries are deliberate and non-negotiable. If a feature request
crosses one of them, it belongs in another project.

## v0.1.0: Minimum Viable Mail Server

**Goal.** A self-hosted operator runs a single binary that signs and sends outbound mail, receives incoming mail with DKIM/SPF/DMARC validation, stores mailboxes, exposes JMAP and IMAP clients, offers a transactional REST API for application use, scores spam natively, authenticates the admin through any OIDC provider and surfaces a minimal SSR webmail and admin dashboard.

**M0 – Foundation.**
- Cargo workspace with seventeen crates, MSRV 1.88, dual MIT/Apache-2.0 licence.
- CI green on three OS, supply chain scan and audit clean.
- Documentation skeleton (`README`, `CONTRIBUTING`, `SECURITY`, `ROADMAP`, `docs/`).
- Single binary `aerogram` with CLI sub-commands (`start`, `init`, `version`, `migrate`).

**M1 – Outbound SMTP and DKIM.**
- SMTP client with STARTTLS, AUTH PLAIN/LOGIN, return-path tracking.
- DKIM signing in Ed25519 (RFC 8463) and RSA-SHA256 (RFC 6376).
- Persistent queue (PostgreSQL) with exponential backoff retry and dead-letter routing.
- Transactional API `POST /v1/messages` returning a message identifier.

**M2 – Events and webhooks.**
- Outgoing events serialised in CloudEvents v1.0 (JSON).
- Webhook delivery signed with HMAC-SHA-256, retried on failure.
- Suppression list per tenant for hard bounces and complaints.
- Optional Hexeract Outbox adapter, gated by a feature flag.

**M3 – Inbound SMTP and validation.**
- SMTP server with STARTTLS, AUTH SASL, command pipelining and MTA-STS (RFC 8461) and TLS-RPT (RFC 8460).
- DKIM verification, SPF (RFC 7208), DMARC (RFC 7489) and optional ARC (RFC 8617).
- Message landing in the metadata store with the corresponding blob persisted to the configured blob backend.

**M4 – Store and search.**
- Metadata store on PostgreSQL with multi-tenant isolation.
- Blob store with filesystem backend by default, S3-compatible backend gated by a feature flag (`s3`).
- Full-text search index over headers and bodies via `tantivy`.

**M5 / M6 – JMAP and IMAP.**
- JMAP server (RFC 8620 + RFC 8621) covering `Mailbox`, `Email`, `Identity`, `EmailSubmission` and `SearchSnippet`.
- IMAP4rev2 (RFC 9051) compat shim exposing the JMAP backend through the IMAP wire protocol, supporting `CAPABILITY`, `LOGIN`, `SELECT`, `FETCH`, `STORE`, `APPEND`, `IDLE` and `SEARCH`.

**M7 – Native antispam.**
- Bayesian classifier with on-disk corpus.
- DNS blocklist lookups with per-tenant policy.
- Greylisting and heuristics, with progressive port of SpamAssassin-compatible rules.

**M8 – Auth and provisioning.**
- OIDC client (RFC 6749 / OpenID Connect Core 1.0) with discovery and JWKS validation.
- SCIM 2.0 (RFC 7643 / 7644) endpoint for cross-IdP mailbox provisioning.
- No-lock-in test: the full stack runs against a standards-compliant OIDC provider, PostgreSQL and RabbitMQ without installing any other Nubster brick.

**M9 – Webmail and admin.**
- Minimal SSR webmail (Askama + HTMX): inbox, message view, send.
- Admin dashboard (Askama + HTMX): tenants, mailboxes, domains, DKIM keys, queue, deliverability events.

**M10 – Release polish.**
- Public Rust API surface frozen for the v0.1 line.
- Security audit and fuzzing harness for SMTP and JMAP parsers.
- Release process documented (`docs/RELEASE_PROCESS.md`).

## v0.2.0: Multi-tenant SaaS surface

**Goal.** Aerogram is operable as a managed multi-tenant SaaS by a single team, with per-tenant IP pools and deliverability dashboards.

- IP pool management with warmup curve and reputation tracking.
- Per-tenant API rate limiting (token bucket).
- Tenant onboarding flow with domain verification (DNS TXT challenge, DKIM key provisioning).
- Deliverability dashboard with bounce rate, complaint rate, open rate and click rate breakdowns.
- Optional storage encryption at rest (per-tenant key).

## v0.3.0: Full-featured webmail and filters

**Goal.** End users have a webmail that holds up against incumbents, and operators can let mailboxes own their own server-side filters.

- Webmail SPA dedicated frontend (Angular), aligned with the Nubster Identity dashboard.
- Sieve filters (RFC 5228) executed server-side per mailbox.
- Full search facets (sender, subject, label, attachment type).
- Calendar invitation rendering (read-only, no CalDAV).

## v0.4.0: Polyglot transports and migration

**Goal.** Cover the rest of the common server-to-server stack and provide a clean migration path from existing self-hosted mail setups.

- LMTP (RFC 2033) for local delivery integration.
- Submission over TLS (RFC 8314) profiles for stricter operators.
- `aerogram import` for migrating existing mailboxes from an IMAP server.
- Pluggable storage backends: object storage providers other than S3-compatible.

## v0.5.0: Polish and stability

**Goal.** Aerogram is usable by external early adopters without hand
holding, the public API is frozen and the documentation lives somewhere
permanent.

- Dedicated documentation site or a section on the Nubster docs portal.
- Onboarding tutorials per integration target (Nubster Identity, generic OIDC provider).
- Performance benchmarks: outbound throughput target on a developer-grade laptop.
- Migration plan towards v1.0 documented.

## Post-1.0 backlog

The items below have been discussed during the design phase but are not
committed to any release. They will only ship if the project gains enough
traction to justify the maintenance cost, and each will require its own
design pass before any code lands.

- **CalDAV and CardDAV** as a separate brick of the Nubster product family.
- **End-to-end encryption** profiles (PGP, S/MIME) at the API level.
- **Federation across Aerogram instances** beyond what standard SMTP provides.
- **AI-assisted triage** as an opt-in plugin via the events bus, never as a default behaviour on user content.
- **Hosted premium tier** and `Nubster Mail` managed offering.

## How this roadmap is maintained

Changes to this document are made by pull request, with a
`docs(roadmap):` Conventional Commit. The scope of v0.1.0 is locked once
the M0 foundation is merged; the scope of later releases stays adjustable
until the previous release ships.

If you spot something missing, redundant or out of scope, open an issue
against the relevant milestone and tag it `discussion`.
