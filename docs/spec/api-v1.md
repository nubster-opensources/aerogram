# API v1

> Pre-alpha. The endpoints described below are the targeted surface for v0.1.0.

## Conventions

- Base path: `/v1/`.
- Authentication: `Authorization: Bearer <jwt>`. The JWT must be signed by the configured OIDC provider; Aerogram validates it against the cached JWKS.
- Authorisation scopes: `mail:send`, `mail:read`, `mail:admin`, `scim:write`.
- Content type: `application/json` for application payloads, `application/scim+json` for SCIM, `multipart/form-data` for raw MIME upload.
- Tenancy: every request is scoped to the `tenant_id` claim resolved from the JWT.
- Idempotency: every `POST` endpoint accepts an `Idempotency-Key` header (UUID). Requests with a previously seen key replay the cached response for 24 hours.
- Errors: RFC 7807 (`application/problem+json`) with `type`, `title`, `status`, `detail`, `instance`.

## Send a message

```http
POST /v1/messages
Authorization: Bearer <jwt-with-mail:send>
Content-Type: application/json
Idempotency-Key: 3c1e8b1f-...

{
  "from": "noreply@example.org",
  "to": ["alice@example.com"],
  "cc": [],
  "bcc": [],
  "reply_to": "support@example.org",
  "subject": "Hello",
  "text": "Plain-text body.",
  "html": "<p>HTML body.</p>",
  "headers": { "X-Tracking-Id": "abc123" },
  "attachments": [
    { "filename": "invoice.pdf", "content_type": "application/pdf", "data_base64": "..." }
  ],
  "tags": ["transactional", "billing"]
}
```

Response `202 Accepted`:

```json
{
  "message_id": "01HZ8K...",
  "status": "queued",
  "accepted_at": "2026-05-26T14:31:09Z"
}
```

## Send raw MIME

```http
POST /v1/messages/raw
Authorization: Bearer <jwt-with-mail:send>
Content-Type: message/rfc822

From: noreply@example.org
To: alice@example.com
Subject: Hello
...
```

Response identical to `POST /v1/messages`.

## Fetch a message status

```http
GET /v1/messages/{message_id}
Authorization: Bearer <jwt-with-mail:read>
```

Response:

```json
{
  "message_id": "01HZ8K...",
  "status": "delivered",            // queued | sent | delivered | bounced | complained
  "attempts": 1,
  "last_attempt_at": "2026-05-26T14:31:13Z",
  "delivered_at": "2026-05-26T14:31:13Z",
  "events": [
    { "type": "queued",    "at": "2026-05-26T14:31:09Z" },
    { "type": "sent",      "at": "2026-05-26T14:31:11Z" },
    { "type": "delivered", "at": "2026-05-26T14:31:13Z" }
  ]
}
```

## List messages

```http
GET /v1/messages?status=bounced&tag=billing&from=2026-05-01&to=2026-05-26&page=1&limit=50
Authorization: Bearer <jwt-with-mail:read>
```

Standard cursor-based pagination via `Link` headers.

## Suppression list

```http
GET    /v1/suppressions
POST   /v1/suppressions   { "address": "alice@example.com", "reason": "complaint" }
DELETE /v1/suppressions/{address}
```

The suppression list is per-tenant. Any address present in the list is rejected by `POST /v1/messages` with `409 Conflict`.

## DKIM keys

```http
GET    /v1/domains/{domain}/dkim
POST   /v1/domains/{domain}/dkim   { "algorithm": "ed25519", "selector": "ag1" }
DELETE /v1/domains/{domain}/dkim/{selector}
```

Returns the DNS record the operator must publish.

## Webhooks

```http
GET    /v1/webhooks
POST   /v1/webhooks   { "url": "https://app.example.org/hooks/mail", "events": ["mail.delivered","mail.bounced"], "secret": "..." }
DELETE /v1/webhooks/{webhook_id}
```

Webhook delivery format:

```http
POST https://app.example.org/hooks/mail
Content-Type: application/cloudevents+json
X-Aerogram-Signature: t=1716729068,v1=8b...

{
  "specversion": "1.0",
  "type": "mail.delivered",
  "source": "aerogram://mail.example.org",
  "id": "01HZ8K...",
  "time": "2026-05-26T14:31:13Z",
  "datacontenttype": "application/json",
  "data": {
    "message_id": "01HZ8K...",
    "to": ["alice@example.com"],
    "delivered_at": "2026-05-26T14:31:13Z",
    "smtp_response": "250 2.0.0 OK"
  }
}
```

Signature verification:

```rust
use hmac::{Hmac, Mac};
use sha2::Sha256;

let mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes())?;
let mac = mac.chain_update(format!("{timestamp}.{raw_body}").as_bytes());
let expected = hex::encode(mac.finalize().into_bytes());
// constant-time compare expected against the v1 segment of the header
```

The timestamp window enforced by Aerogram is 5 minutes; older signatures are rejected by the SDK.

## SCIM 2.0

`/scim/v2/` exposes the standard SCIM endpoints: `/Users`, `/Groups`, `/ResourceTypes`, `/Schemas`, `/ServiceProviderConfig`. The schemas are vanilla SCIM 2.0; no custom extensions in v0.1.0.

## Rate limits

Default token bucket per tenant, configurable via `[api.rate_limit]`. Exceeded calls return `429 Too Many Requests` with `Retry-After`.

## Error format (RFC 7807)

```json
{
  "type": "https://nubster.com/aerogram/errors/suppressed-address",
  "title": "Recipient is suppressed",
  "status": 409,
  "detail": "alice@example.com is on the suppression list for tenant 4f0b...",
  "instance": "/v1/messages/01HZ8K..."
}
```

## Pre-alpha disclaimer

Every endpoint above is subject to change until v0.1.0 ships. The wire format becomes part of the public API per [SEMVER_POLICY.md](../SEMVER_POLICY.md) at that release.
