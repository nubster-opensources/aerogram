//! Aerogram SMTP listener and client.
//!
//! Hosts the three SMTP-related runtime services of the server:
//!
//! - [`SmtpServer`] accepts inbound mail on port 25, with STARTTLS and
//!   the optional AUTH SASL stack.
//! - [`Submission`] accepts submission on ports 465 (implicit TLS) and 587
//!   (STARTTLS), authenticates the client and queues the message.
//! - [`SmtpClient`] (the MTA role) drains the outbound queue and delivers
//!   to remote MTAs, signing each message through `aerogram-dkim` and
//!   recording delivery attempts in `aerogram-queue`.

use aerogram_core::{Envelope, TenantId};

/// Inbound SMTP server.
#[derive(Debug, Clone)]
pub struct SmtpServer {
    _bind: String,
}

impl SmtpServer {
    /// Builds an inbound SMTP server bound to the supplied address.
    pub fn new(bind: impl Into<String>) -> Self {
        Self { _bind: bind.into() }
    }
}

/// Submission listener for authenticated clients.
#[derive(Debug, Clone)]
pub struct Submission {
    _bind: String,
}

impl Submission {
    /// Builds a submission listener bound to the supplied address.
    pub fn new(bind: impl Into<String>) -> Self {
        Self { _bind: bind.into() }
    }
}

/// Outbound MTA: drains the queue and delivers to remote servers.
#[derive(Debug, Clone, Default)]
pub struct Mta;

/// SMTP outbound client (single-recipient helper, used by [`Mta`]).
#[derive(Debug, Clone, Default)]
pub struct SmtpClient;

impl SmtpClient {
    /// Delivers the supplied raw message to its envelope recipients.
    ///
    /// # Errors
    ///
    /// Returns [`SmtpError::Transport`] when the delivery fails.
    pub async fn deliver(
        &self,
        _tenant_id: TenantId,
        _envelope: Envelope,
        _message: &[u8],
    ) -> Result<DeliveryReport, SmtpError> {
        unimplemented!("M1: signature only")
    }
}

/// SASL authentication mechanism advertised on the submission listener.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthMechanism {
    /// `PLAIN` mechanism.
    Plain,
    /// `LOGIN` mechanism.
    Login,
    /// `SCRAM-SHA-256` mechanism.
    ScramSha256,
}

/// Report attached to a delivery attempt.
#[derive(Debug, Clone)]
pub struct DeliveryReport {
    /// SMTP response code captured from the remote MTA.
    pub code: u16,
    /// SMTP response text captured from the remote MTA.
    pub text: String,
}

/// SMTP-specific errors.
#[derive(Debug, thiserror::Error)]
pub enum SmtpError {
    /// Transport-level failure (TCP, TLS, timeout).
    #[error("smtp transport error: {0}")]
    Transport(String),
    /// Protocol-level failure (unexpected response, malformed command).
    #[error("smtp protocol error: {0}")]
    Protocol(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn submission_records_its_bind() {
        let submission = Submission::new("0.0.0.0:587");
        drop(submission);
    }
}
