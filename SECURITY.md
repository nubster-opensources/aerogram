# Security policy

## Supported versions

Aerogram follows the [semver policy](docs/SEMVER_POLICY.md). During the 0.x phase, only the latest minor release receives security fixes.

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |
| < 0.1   | :x:                |

The supported window will be widened once Aerogram reaches 1.0.

## Threat model

Aerogram is a mail server. Its threat model is broader than a typical Rust library because the binary handles untrusted input from the public Internet (incoming SMTP), stores private user material (mailbox content, DKIM private keys, OIDC client secrets) and signs outbound traffic on behalf of its tenants.

The following attack surfaces are explicitly in scope:

- **Inbound SMTP processing** (RFC 5321, 5322, 6531), including STARTTLS downgrade, command pipelining smuggling and address parsing on hostile MIME.
- **DKIM signing and verification** (RFC 6376, 8463), including key handling, header canonicalization edge cases and signature replay.
- **SPF and DMARC evaluation** (RFC 7208, 7489), including macro expansion and `void lookups` budget exhaustion.
- **JMAP and IMAP servers** (RFC 8620, 8621, 9051), including session fixation, cross-tenant data access through forged identifiers and uncontrolled memory growth on large `FETCH` requests.
- **Webhook signing** (HMAC SHA-256), including replay and timing attacks on signature verification.
- **OIDC and JWKS validation** in `aerogram-auth`, including key confusion, `alg=none` and unsigned `kid` substitution.
- **SCIM 2.0 provisioning** endpoint, including authorization bypass and tenant isolation.
- **Multi-tenant isolation** at the store, queue and search-index layers, including cross-tenant blob access through forged identifiers.
- **Antispam evasion** (rule bypass, Bayesian poisoning), in `aerogram-spam`.

The following are assumed safe by the threat model:

- The PostgreSQL database, the blob store (filesystem or S3-compatible) and the local filesystem on the host running Aerogram are trusted.
- Operators with shell access to the host running Aerogram are trusted.
- The Rust standard library, `tokio`, `rustls`, `sqlx` and other supply-chain dependencies are trusted up to the vulnerabilities published in their respective advisories.

## Reporting a vulnerability

If you find a security vulnerability in Aerogram, please **do not** open a public GitHub issue. Disclosure rules:

1. Email a detailed report to **security@nubster.com** with the subject prefix `[aerogram security]`.
2. The report should include:
   - A description of the vulnerability and the attacker model.
   - Affected versions and crates.
   - Reproduction steps or a proof of concept.
   - The impact you anticipate (data leak, denial of service, privilege escalation, header forgery, open relay, etc.).
   - Suggested mitigation if you have one.
3. You will receive an acknowledgement within **7 calendar days**. If you do not, please follow up at the same address.
4. We will work with you to validate, scope and remediate the issue. A coordinated disclosure timeline will be agreed in writing. The default embargo period is **90 days** from acknowledgement.
5. Once a fix is published, you will be credited in the release notes unless you prefer to remain anonymous.

## Encrypted reporting

If your report includes confidential proof-of-concept material, please encrypt it with the Nubster security GPG key. The fingerprint and public key are published at <https://nubster.com/.well-known/security.txt> (once Nubster publishes them).

## Out of scope

The following are explicitly **out of scope** for vulnerability reports:

- Issues in unsupported versions.
- Vulnerabilities in third-party dependencies that are already publicly disclosed and tracked upstream. Report them to the upstream project.
- Reports based on theoretical attacks without a working proof of concept.
- Misconfiguration by the operator (open relay enabled, missing TLS certificate, DKIM key world-readable on the filesystem). These are documented hazards, not vulnerabilities.
- Reports requiring an attacker already in possession of valid administrative credentials.
- Denial of service achievable only by malicious operators of the database or blob store the server connects to. The threat model assumes trusted infrastructure.
- Deliverability complaints (mail flagged as spam by a remote receiver). These are operational and belong on the operator, not the software.

## Public security advisories

Confirmed and fixed vulnerabilities are published on the GitHub Security Advisories page of the repository. RustSec advisories are also coordinated for severe issues when applicable.
