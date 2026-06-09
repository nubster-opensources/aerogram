//! Aerogram binary entry point.
//!
//! Thin wrapper around `aerogram-server`. Parses the command-line, sets up
//! logging through `tracing-subscriber` and hands control to the server
//! lifecycle. Every subcommand maps to a dedicated workflow on the server
//! side; the binary itself contains no business logic.

use std::path::PathBuf;

use clap::{Parser, Subcommand};

/// Top-level CLI for the `aerogram` binary.
#[derive(Debug, Parser)]
#[command(
    name = "aerogram",
    version,
    about = "Sovereign Rust email server: SMTP, JMAP, IMAP, transactional API."
)]
pub struct Cli {
    /// Subcommand to execute.
    #[command(subcommand)]
    pub command: Command,
}

/// Subcommands exposed by the binary.
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Starts the server using the supplied configuration file.
    Start {
        /// Path to the TOML configuration file.
        #[arg(long, default_value = "/etc/aerogram/aerogram.toml")]
        config: PathBuf,
    },
    /// Generates a default configuration and the initial DKIM keypair.
    Init {
        /// Domain to bootstrap.
        #[arg(long)]
        domain: String,
        /// PostgreSQL URL to write into the generated configuration.
        #[arg(long)]
        postgres_url: String,
        /// Output path for the generated TOML configuration file.
        #[arg(long, default_value = "/etc/aerogram/aerogram.toml")]
        config_out: PathBuf,
    },
    /// Applies the database migrations against the configured PostgreSQL.
    Migrate {
        /// Path to the TOML configuration file.
        #[arg(long, default_value = "/etc/aerogram/aerogram.toml")]
        config: PathBuf,
    },
    /// Prints the binary version and exits.
    Version,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _cli = Cli::parse();
    tracing_subscriber::fmt::init();
    Ok(())
}
