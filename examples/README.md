# Aerogram examples

> Pre-alpha. The directories below describe the runnable examples targeted for v0.1.0. None of them is functional yet; treat them as design artefacts until the milestones quoted next to each example close.

Aerogram ships three reference deployments that exercise the public surface of the server end to end. They double as integration tests and as documentation for self-hosted operators.

| Example | Targeted milestone | Demonstrates |
| --- | --- | --- |
| [`01-send-transactional/`](./01-send-transactional/) | M1 - M2 | Transactional API for outbound delivery, webhook receiver, DKIM signing. |
| [`02-self-hosted-oidc/`](./02-self-hosted-oidc/) | M8 | The no-vendor-lock-in commitment: Aerogram authenticating against any standards-compliant OIDC provider, SCIM 2.0 provisioning, no Nubster brick in the loop. |
| [`03-full-mail-server/`](./03-full-mail-server/) | M10 | A full stack with inbound and outbound SMTP, JMAP, IMAP, antispam and admin SSR. Closest to the v0.1.0 reference deployment. |

Each example owns a self-contained `docker-compose.yml` and a `README.md` describing the prerequisites, the bring-up sequence and the validation checklist.
