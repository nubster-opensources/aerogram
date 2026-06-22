# Example 02: self-hosted with your own OIDC provider (no vendor lock-in)

> Targeted for milestone **M8**. Not runnable yet.

This example is the reference proof of the no-vendor-lock-in commitment documented in [`docs/design/interop.md`](../../docs/design/interop.md). It stands up Aerogram with **no Nubster brick** in the loop: authentication is delegated to your own OIDC provider, PostgreSQL holds the store, the filesystem holds the blobs.

The bring-up sequence is:

1. Configure your OIDC provider to expose a discovery document at `/.well-known/openid-configuration` and register an `aerogram-admin` client with the `mail:admin` scope. Set the issuer URL in `aerogram.toml` under `[auth.oidc] issuer`.
2. `docker compose up -d postgres aerogram` brings Aerogram and its store online. The OIDC provider is expected to run externally or as a service you add to `docker-compose.yml`.
3. The Aerogram configuration points at the OIDC issuer URL (no other identity-related service involved).
4. The validation checklist exercises: admin sign-in via the OIDC provider, mailbox provisioning via SCIM 2.0, sending a message via the API, receiving the delivery webhook.

The directory will contain:

- `docker-compose.yml` wiring Aerogram and PostgreSQL. The OIDC provider service is a commented-out placeholder: replace with your own OIDC provider, configured via the OIDC issuer URL.
- `oidc-provider/` placeholder directory with a `README.md` explaining the required provider configuration (discovery endpoint, client registration, scopes).
- `aerogram.toml` configured against a placeholder OIDC issuer URL (`https://id.example.org`).
- `validate.sh` running the checklist.

The example will be referenced by the M8 integration test of `aerogram-auth` and is part of the public no-vendor-lock-in test suite.
