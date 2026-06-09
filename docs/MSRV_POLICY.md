# Minimum Supported Rust Version (MSRV) policy

The current MSRV is **Rust 1.88** (stable channel).

The MSRV is pinned in `rust-toolchain.toml` at the repository root and declared in every workspace crate via `rust-version = "1.88"`.

## How the MSRV evolves

- Aerogram does not commit to supporting Rust versions older than 1.88.
- An MSRV bump is treated as a **minor** version bump per the [semver policy](SEMVER_POLICY.md). For example, raising the MSRV from 1.88 to 1.92 ships in a `0.X.0` release (or `X.0.0` once at 1.0).
- The current MSRV is documented in CHANGELOG.md under the `Changed` section of the release that bumps it.

## Why we pick the floor we pick

- **1.88** is required because Aerogram uses Rust edition 2024 features and the stabilised `async fn` in traits (via `trait_variant::make(Send)`).
- Future bumps will be driven by concrete features the server needs (for example an upcoming GAT stabilisation, an `async closure` improvement, a `rustls` API requiring a newer compiler), not by chasing the latest stable.

## How we verify the MSRV in CI

The repository CI pins `rust-toolchain.toml` to `1.88.0`. The `Format`, `Clippy` and `Build and test` jobs all run on this exact toolchain, which guarantees that nothing newer slips in. A dedicated MSRV check job (`cargo +1.88 check --workspace --all-features`) runs on every pull request as an early warning on breakage introduced by upstream dependency changes.

## Downstream impact

If you depend on an Aerogram crate, the dependency resolver will refuse to compile your project on a Rust version older than the MSRV. In your `Cargo.toml`, you can pin a lower MSRV than ours only if you also pin Aerogram to a version that supports it, which is documented per release in CHANGELOG.md.
