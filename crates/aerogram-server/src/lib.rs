//! Aerogram composition root.
//!
//! Owns the runtime lifecycle: parses the configuration, wires every
//! functional crate into the same Tokio runtime, spawns the per-listener
//! and per-worker tasks, exposes a single graceful shutdown signal and
//! propagates errors back to the binary entry point. The crate is the
//! only one allowed to depend on every other crate of the workspace.

use aerogram_config::Config;

/// Top-level runnable server.
#[derive(Debug)]
pub struct Server {
    _config: Config,
}

impl Server {
    /// Builds the server from a parsed [`Config`].
    #[must_use]
    pub fn from_config(config: Config) -> Self {
        Self { _config: config }
    }

    /// Runs the server until [`ShutdownSignal::shutdown`] is invoked.
    ///
    /// # Errors
    ///
    /// Returns [`anyhow::Error`] when any listener or worker fails to
    /// start.
    pub async fn run(self, _shutdown: ShutdownSignal) -> anyhow::Result<()> {
        unimplemented!("M0: signature only")
    }
}

/// Builder that produces a [`Server`] from a configuration file path.
#[derive(Debug, Default)]
pub struct ServerBuilder;

impl ServerBuilder {
    /// Builds a [`Server`] from the configuration at the supplied path.
    ///
    /// # Errors
    ///
    /// Returns [`anyhow::Error`] when parsing or validation fails.
    pub fn from_path(&self, _path: &std::path::Path) -> anyhow::Result<Server> {
        unimplemented!("M0: signature only")
    }
}

/// Holder for the lifecycle hooks exposed by the server.
#[derive(Debug, Default)]
pub struct Lifecycle;

/// Graceful shutdown signal, driven by `SIGINT` or `SIGTERM`.
#[derive(Debug, Clone, Default)]
pub struct ShutdownSignal {
    _placeholder: (),
}

impl ShutdownSignal {
    /// Creates a new shutdown signal.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Triggers the shutdown.
    pub fn shutdown(&self) {
        // M0 placeholder; the real implementation publishes through a
        // `tokio_util::sync::CancellationToken`.
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shutdown_signal_is_default_constructible() {
        let signal = ShutdownSignal::new();
        signal.shutdown();
    }
}
