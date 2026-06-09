# Example 03: full mail server

> Targeted for milestone **M10**. Not runnable yet.

This example is the closest to the v0.1.0 reference deployment: a complete mail server with inbound SMTP, outbound SMTP, JMAP, IMAP, native antispam, admin SSR and webmail SSR, all behind a reverse proxy.

The bring-up sequence is:

1. `docker compose up -d` brings up Aerogram, PostgreSQL, your OIDC provider (replace with your own OIDC provider, configured via the OIDC issuer URL; Nubster Identity available as a convenient option) and a reverse proxy terminating TLS.
2. The operator publishes the DNS records printed by `aerogram init` (MX, DKIM, SPF, DMARC, MTA-STS, TLS-RPT).
3. The validation checklist exercises: receive a message from an external sender (with DKIM, SPF and DMARC checks), send a message to an external recipient (with DKIM signing), open the inbox via the SSR webmail, connect a desktop client over IMAP, and access the admin dashboard.

The directory will contain:

- `docker-compose.yml` wiring Aerogram, PostgreSQL, your OIDC provider and Caddy.
- `aerogram.toml` configured for the reference deployment.
- `Caddyfile` terminating TLS on ports 465, 587, 993 and 8443.
- `validate.sh` exercising the full checklist.

The example will be referenced by the M10 release polish milestone and by the public ROADMAP as the closing acceptance criterion for v0.1.0.
