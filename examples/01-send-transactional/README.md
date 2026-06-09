# Example 01: send a transactional message

> Targeted for milestone **M1–M2**. Not runnable yet.

This example shows the minimal path a backend application takes to send a transactional message through Aerogram:

1. Boot Aerogram with the bundled `docker-compose.yml` (Aerogram + PostgreSQL + a tiny webhook receiver).
2. Mint a JWT scoped to `mail:send`.
3. `POST /v1/messages` with a JSON payload.
4. Observe the `mail.delivered` webhook hitting the local receiver.

The directory will contain:

- `docker-compose.yml` wiring Aerogram, PostgreSQL and a webhook receiver.
- `aerogram.toml` with a minimal configuration (filesystem blob store, no inbound SMTP).
- `send.sh` showing the curl call.
- `receiver.rs` (axum-based) printing every received webhook with signature verification.

The example will be referenced by the M1 and M2 integration tests of `aerogram-api`.
