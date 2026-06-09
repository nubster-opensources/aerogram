# Semantic Versioning policy

Aerogram follows [Semantic Versioning 2.0.0](https://semver.org/) with explicit conventions for the 0.x phase.

## 0.x phase (pre-1.0)

While the major version is 0, breaking changes are allowed on a minor version bump:

- `0.1.x` -> `0.1.y` (patch): bug fixes, performance improvements, internal refactors, additive non-breaking changes. No public API change observable by a downstream user.
- `0.x.y` -> `0.X.0` (minor): may introduce breaking changes. Removed items must have been deprecated for at least one previous minor release whenever feasible.

Reasoning: every component of Aerogram is shipped early to gather feedback. Locking ourselves into Semver-strict semantics before the API surface is stable would prevent the changes we know we still need.

## 1.0 and beyond

Once 1.0 is reached, Aerogram commits to strict Semver:

- Major (`X.0.0`): breaking changes to the public API.
- Minor (`1.Y.0`): backwards-compatible additions.
- Patch (`1.x.Z`): backwards-compatible bug fixes.

## Public API definition

The public API consists of every item that is reachable from a crate's root through `pub` re-exports, except items marked `#[doc(hidden)]`. This includes:

- Public types, traits, functions, constants and modules.
- Trait method signatures and associated types.
- Public re-exports from sibling crates (the facade crate `aerogram` re-exports curated items).
- The on-disk schema produced by `aerogram migrate`. Schema changes that require migration are treated as breaking changes.
- The wire format of the transactional REST API under `/v1/`, the JMAP capabilities exposed and the IMAP capability list.
- The webhook payload schema and HMAC signature scheme.
- The CloudEvents payload schema for outgoing events.

Items that are explicitly NOT part of the public API:

- Anything under a module annotated `#[doc(hidden)]`.
- Test-only helpers under `#[cfg(test)]`.
- Behaviour of internal SQL statements (the `WHERE` clauses, the `LIMIT`, the index usage) provided that observable semantics (multi-tenant isolation, at-least-once webhook delivery, partial ordering of deliveries) are preserved.
- The internal layout of the blob store on disk or in object storage. Operators must use the public CLI and API to interact with stored mail.

## Deprecation cycle

When an item is to be removed:

1. The item is marked `#[deprecated(since = "0.X.0", note = "use Y instead")]` in the release that introduces the replacement.
2. The deprecated item continues to compile and run unchanged for the entire next minor cycle.
3. The item is removed in the minor release after that, at the earliest. Removal is documented in CHANGELOG.md under `Removed` for that version.

## Breaking change documentation

Every breaking change is announced in CHANGELOG.md under `Changed` or `Removed`, with:

- The new signature or replacement.
- A migration snippet when the change is non-mechanical.
- A link to the relevant pull request or discussion when context is useful.

## MSRV

The MSRV (Minimum Supported Rust Version) is governed by [MSRV_POLICY.md](MSRV_POLICY.md). An MSRV bump is treated as a minor version bump.
