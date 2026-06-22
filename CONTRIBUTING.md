# Contributing to Aerogram

Aerogram is currently in **pre-alpha**. The repository is public from day one to capture the name and make the design discussion visible, but the public API is not stable yet. The information below describes the conventions that will apply once the project opens for contributions.

## Conventions

Aerogram follows the Nubster general coding standards documented in [nubster-docs](https://github.com/nubster-opensources/nubster-docs/tree/main/docs/reference/coding-standards). In short:

- **Trunk-Based Development**, feature branches `feature/<issue>-<slug>` from `main`, never commit directly on `main`.
- **Conventional Commits**, all commit messages follow the `type(scope): description` format, enforced by `cog verify` in the commit-msg hook.
- **Rust style**, workspace lints `clippy::all` and `clippy::pedantic` set to `deny`, MSRV pinned in `rust-toolchain.toml` and `Cargo.toml`.
- **No competitor mentions**, the source code, commit messages, pull requests and documentation never name competing tools or commercial mail providers.
- **English everywhere**, rustdoc comments, public types, commit messages, issues and project documentation are all written in English.
- **No vendor lock-in**, every external integration (auth, events, audit) goes through an open standard (OIDC, SCIM, CloudEvents, HMAC). Nubster Identity and Hexeract are the default convenient options, never a hard dependency.

## Local setup (when the project opens)

```bash
# Pin the Rust toolchain via rustup
rustup show

# Install local git hooks
lefthook install

# Run tests
cargo test --workspace --all-features
```

## Discussion before code

Until v0.1.0, all design decisions go through a `discussion/` thread on the repository before any pull request is opened. This includes the public traits, the SMTP/JMAP/IMAP server contracts, the mailbox model, the queue schema, the antispam scoring policy and the webhook payload format.

## Contributor License Agreement

Contributions to this project are governed by the Nubster Contributor License Agreement, hosted at [github.com/nubster-opensources/cla](https://github.com/nubster-opensources/cla).

On your first pull request, the CLA Assistant bot will automatically prompt you to sign the CLA. Once signed, your signature applies to all current and future contributions to any `nubster-opensources` project.

The CLA is a license grant (not a copyright assignment): you keep the copyright on your contributions and grant Nubster a broad license to use, sub-license, and re-license them.

## License

By contributing, you agree that your contributions are dual-licensed under the [MIT License](./LICENSE-MIT) and the [Apache License, Version 2.0](./LICENSE-APACHE), at the user's option.

Copyright © Nubster.
