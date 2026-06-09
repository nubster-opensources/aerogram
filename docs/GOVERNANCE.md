# Governance

Aerogram is an open-source project sponsored by Nubster under the `nubster-opensources` organisation. It is developed as part of the Nubster product family.

## Roles

### BDFL

The project follows a Benevolent Dictator For Life (BDFL) model.

- **BDFL:** Pierrick Fonquerne, Founder.
- The BDFL holds the final decision on the public API, the project roadmap, the release cadence, every semver level decision, every MSRV bump, and any change to this governance document.
- The BDFL takes input from maintainers and the community, but reserves the right to make the call when consensus cannot be reached.

### Maintainers

Maintainers are contributors with write access to the repository. Their responsibilities:

- Triage incoming issues and pull requests.
- Review and approve pull requests against `main`.
- Cut releases following [docs/RELEASE_PROCESS.md](RELEASE_PROCESS.md).
- Enforce the [Code of Conduct](../CODE_OF_CONDUCT.md).

Maintainers are appointed by the BDFL.

### Contributors

Anyone who opens an issue, comments, or sends a pull request. Contributors do not need an invitation to participate; the only requirement is that they follow the Code of Conduct.

## Development workflow

Aerogram uses Trunk-Based Development.

- `main` is the single long-lived branch and is always releasable.
- Short-lived branches are created from `main` with the prefixes `feature/`, `fix/`, `chore/`, or `docs/`.
- Every change reaches `main` through a pull request.
- **At least one maintainer review** is required before a pull request can be merged into `main`.
- **No force push to `main`** under any circumstance. History on `main` is append-only.
- CI must be green before merge. Failing CI is a blocker, never a "merge anyway".
- Pull requests are merged with `--no-ff` to preserve the integration commit. Squash merges are not used.

## Decision making

- **API and architectural decisions:** taken by the BDFL after public discussion in the relevant issue or pull request. Significant changes are documented in `docs/` (this folder) or in CHANGELOG.md when they ship.
- **Semver level (patch / minor / major):** taken by the BDFL with input from maintainers, following the rules in [docs/SEMVER_POLICY.md](SEMVER_POLICY.md).
- **MSRV bumps:** taken by the BDFL with input from maintainers, following the rules in [docs/MSRV_POLICY.md](MSRV_POLICY.md). An MSRV bump is treated as a minor version bump.
- **Security:** vulnerability reports follow [SECURITY.md](../SECURITY.md). The BDFL coordinates disclosure timelines.

## Code of Conduct

All participants are bound by the [Code of Conduct](../CODE_OF_CONDUCT.md). Enforcement decisions are made by the BDFL, in consultation with maintainers when appropriate.

## Amending this document

Changes to this governance document are proposed via pull request and require BDFL approval to merge.
