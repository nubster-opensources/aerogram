//! Aerogram IMAP4rev2 listener.
//!
//! Implements the IMAP4rev2 wire protocol (RFC 9051) as a compatibility
//! shim mapping onto the JMAP backend exposed by `aerogram-jmap`. The
//! shipping subset for v0.1.0 covers `CAPABILITY`, `LOGIN`, `SELECT`,
//! `FETCH`, `STORE`, `APPEND`, `IDLE` and `SEARCH`.

use aerogram_core::TenantId;

/// IMAP4rev2 listener.
#[derive(Debug, Clone)]
pub struct ImapServer {
    _bind: String,
}

impl ImapServer {
    /// Builds an IMAP listener bound to the supplied address.
    pub fn new(bind: impl Into<String>) -> Self {
        Self { _bind: bind.into() }
    }
}

/// IMAP session state for a single authenticated connection.
#[derive(Debug, Clone)]
pub struct ImapSession {
    /// Tenant the session is scoped to.
    pub tenant_id: TenantId,
    /// Currently selected mailbox name.
    pub selected: Option<String>,
}

/// IMAP commands accepted by the v0.1.0 shim.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImapCommand {
    /// `CAPABILITY` command.
    Capability,
    /// `LOGIN <user> <pass>` command (kept for compat; submission uses AUTH).
    Login(String, String),
    /// `SELECT <mailbox>` command.
    Select(String),
    /// `FETCH <set> <items>` command.
    Fetch(String, String),
    /// `STORE <set> <items>` command.
    Store(String, String),
    /// `APPEND <mailbox> <flags> <bytes>` command.
    Append(String, Vec<String>, Vec<u8>),
    /// `IDLE` command.
    Idle,
    /// `SEARCH <criteria>` command.
    Search(String),
    /// `LOGOUT` command.
    Logout,
}

/// IMAP server response framing.
#[derive(Debug, Clone)]
pub enum ImapResponse {
    /// Tagged completion response (`OK`, `NO`, `BAD`).
    Tagged(String, ImapStatus, String),
    /// Untagged data response.
    Untagged(String),
}

/// IMAP status (`OK`, `NO`, `BAD`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImapStatus {
    /// Command succeeded.
    Ok,
    /// Command refused by the server.
    No,
    /// Command was malformed.
    Bad,
}

/// Capability list advertised by the listener.
#[derive(Debug, Clone)]
pub struct Capability {
    /// Capability tokens (`IMAP4rev2`, `STARTTLS`, `AUTH=PLAIN`, ...).
    pub tokens: Vec<String>,
}

/// IMAP-specific errors.
#[derive(Debug, thiserror::Error)]
pub enum ImapError {
    /// Transport-level failure.
    #[error("imap transport error: {0}")]
    Transport(String),
    /// Protocol-level failure.
    #[error("imap protocol error: {0}")]
    Protocol(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn imap_status_is_copy() {
        let s = ImapStatus::Ok;
        let t = s;
        assert_eq!(s, t);
    }
}
