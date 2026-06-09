//! Aerogram repository tooling.
//!
//! Internal `cargo xtask` binary providing the maintenance commands the
//! release process and the documentation pipeline need. The crate is
//! marked `publish = false` and never ships to crates.io.

use clap::{Parser, Subcommand};

/// Top-level CLI for the `xtask` binary.
#[derive(Debug, Parser)]
#[command(name = "xtask", about = "Aerogram repository tooling.")]
struct Cli {
    /// Subcommand to execute.
    #[command(subcommand)]
    command: Command,
}

/// Available subcommands.
#[derive(Debug, Subcommand)]
enum Command {
    /// Runs the pre-flight checks expected by `scripts/release.sh`.
    ReleaseCheck,
    /// Regenerates the local test fixtures from the spec files.
    Fixtures,
    /// Generates the auxiliary documentation artefacts.
    GenDocs,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::ReleaseCheck => {
            println!("xtask: release-check is a placeholder until M10.");
        }
        Command::Fixtures => {
            println!("xtask: fixtures is a placeholder until M3.");
        }
        Command::GenDocs => {
            println!("xtask: gen-docs is a placeholder until M9.");
        }
    }
    Ok(())
}
