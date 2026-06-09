# Release process

Aerogram uses a three-layer combo to ship new versions to crates.io. Whatever surface you choose, the underlying flow is identical: bump versions, graduate CHANGELOG, pre-flight checks, open a release prep PR, then push a tag that fires the publish workflow.

## Surfaces

### Surface 1: GitHub UI (no CLI required)

Use this when you want to bump from your browser, or when you do not have a local Rust toolchain handy.

1. Open <https://github.com/nubster-opensources/aerogram/actions/workflows/bump.yml>.
2. Click **Run workflow**.
3. Pick the **level** input:
   - `patch`: `0.1.0` → `0.1.1` (bug fixes)
   - `minor`: `0.1.0` → `0.2.0` (breaking changes allowed in 0.x per [SEMVER_POLICY.md](SEMVER_POLICY.md))
   - `major`: `1.2.3` → `2.0.0` (breaking changes in 1.x+)
   - explicit `x.y.z`: for example `0.3.0`
4. The workflow runs `scripts/release.sh` in CI and opens a release prep PR.
5. Review the PR, merge it, then follow [Tagging](#tagging).

### Surface 2: local script

Use this when you want full control over the pre-flight (run tests against your local Docker, tweak CHANGELOG manually, etc.).

```sh
./scripts/release.sh patch          # or minor / major / 0.3.0
```

Requirements (must be installed on your machine):

- `bash`, `git`, `python3`, `gh`
- `cargo` and the `cargo-release` subcommand pinned to the latest version compatible with our MSRV (`cargo-release 1.1+` requires Rust 1.91+, so we stay on the 0.25 series): `cargo install --locked cargo-release@0.25.20`

The script:

1. Refuses to run if you are not on `main` or if the working tree is dirty.
2. Pulls `origin/main`.
3. Computes the target version from the bump level.
4. Creates a `release/v<TARGET>-prep` branch.
5. Graduates `CHANGELOG.md`: moves `[Unreleased]` body under a new `[<TARGET>] - <DATE>` section, refreshes the link refs.
6. Runs `cargo release <LEVEL> --workspace --execute --no-confirm` which bumps every `Cargo.toml` version field (path-deps included) in a single consolidated commit.
7. Runs `cargo fmt --check`, `clippy` strict, full test suite.
8. Pushes the branch and opens a PR via `gh`.

### Surface 3: power-user, cargo-release direct

For one-off bumps where you do not need the CHANGELOG graduation or the PR opening:

```sh
cargo release patch --workspace --execute --no-confirm
```

This is what `scripts/release.sh` calls under the hood. You will need to graduate the CHANGELOG manually and open the PR yourself.

## Tagging

After the release prep PR is merged into `main`, push the tag manually:

```sh
git checkout main
git pull origin main
git tag -a v<TARGET> -m "v<TARGET>"
git push origin v<TARGET>
```

The tag push triggers [`.github/workflows/release.yml`](../.github/workflows/release.yml) which publishes the workspace crates to crates.io in dependency order (with 30 s sleeps between each to let the crates.io index propagate) and creates a GitHub Release whose notes are extracted from the `[<TARGET>]` section of `CHANGELOG.md`.

Tagging is deliberately a manual step so the human reviewing the PR is also the one who triggers the publish, with full awareness of what is about to leave the workshop.

## Publication order

The seventeen workspace crates are published in dependency order. Crates that depend on others must wait for them to be visible on the crates.io index:

1. `aerogram-core`
2. `aerogram-config`
3. `aerogram-dkim`
4. `aerogram-spf-dmarc`
5. `aerogram-queue`
6. `aerogram-store`
7. `aerogram-search`
8. `aerogram-spam`
9. `aerogram-events`
10. `aerogram-auth`
11. `aerogram-smtp`
12. `aerogram-jmap`
13. `aerogram-imap`
14. `aerogram-api`
15. `aerogram-admin`
16. `aerogram-server`
17. `aerogram` (binary facade)

The `xtask` crate is internal to the repository and is never published.

## What the bump script does NOT do

- It does not publish to crates.io. The tag does, via `release.yml`.
- It does not create the GitHub Release. The tag does.
- It does not edit your `[Unreleased]` items. Whatever you wrote there is preserved verbatim under the new `[<TARGET>]` section.
- It does not skip pre-flight checks. If `cargo fmt` or `clippy` or the test suite fails, the bump aborts.

## Failure modes

- **`error: must be on main`**: switch back to main, then retry.
- **`error: working tree must be clean`**: commit or stash your local changes.
- **`error: branch release/vX.Y.Z-prep already exists locally`**: a previous bump for the same version is still around. Delete it (`git branch -D release/vX.Y.Z-prep`) or pick a different target.
- **Pre-flight failure (fmt / clippy / test)**: fix the failure on `main` first via a normal PR, then retry the bump.
- **`gh pr create` fails**: probably an auth issue. Verify `gh auth status`. In CI the `GITHUB_TOKEN` is provided automatically.
- **CHANGELOG section not found**: the Python script expects `## [Unreleased]` followed by `## [` somewhere later. If you renamed `[Unreleased]` or removed the next section, the script aborts.

## Adding it to the project

`release.toml` lives at the repo root and configures cargo-release. The script lives at `scripts/release.sh` and is executable. The workflow lives at `.github/workflows/bump.yml`. None of these files affect runtime crates; they are purely repository tooling.
