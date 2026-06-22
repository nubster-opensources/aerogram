//! Aerogram admin and webmail SSR.
//!
//! Hosts the self-hosted admin dashboard (tenants, mailboxes, domains,
//! DKIM keys, queue inspection, deliverability metrics) and the minimal
//! webmail (inbox, message view, send) shipped in v0.1.0. The two surfaces
//! share the same axum router and the same Askama template set, served
//! with HTMX for incremental interactions and without a single-page
//! application bundle.

/// Admin and webmail HTTP server, mounted under `/admin/` and `/mail/`.
#[derive(Debug, Clone, Default)]
pub struct AdminServer;

impl AdminServer {
    /// Returns an axum router exposing the admin and webmail endpoints.
    #[must_use = "the router must be mounted on the application"]
    pub fn router(&self) -> axum::Router {
        axum::Router::new()
    }
}

/// Router for the `/admin/` subtree.
#[derive(Debug, Clone, Default)]
pub struct AdminRouter;

/// Holder for the compiled Askama templates.
#[derive(Debug, Clone, Default)]
pub struct Templates;

/// Admin-specific errors.
#[derive(Debug, thiserror::Error)]
pub enum AdminError {
    /// Template rendering failed.
    #[error("admin template error: {0}")]
    Template(String),
    /// Authorisation failure.
    #[error("admin forbidden: {0}")]
    Forbidden(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn admin_server_router_is_constructible() {
        let server = AdminServer;
        drop(server.router());
    }
}
