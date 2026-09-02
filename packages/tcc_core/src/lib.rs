//! TCC Launcher core logic.

use tcc_auth::AuthService;
use tcc_events::EventBus;
use tcc_net::RequestClient;

pub struct LauncherCore {
    pub auth: AuthService,
    pub events: EventBus,
}

impl LauncherCore {
    pub async fn new() -> anyhow::Result<Self> {
        let net = RequestClient::new();
        let events = EventBus::new();
        let auth = AuthService::load(net, events.clone()).await?;

        Ok(Self { auth, events })
    }
}

/// Error type shared by launcher-level hooks and queries.
#[derive(Debug, Clone, thiserror::Error)]
pub enum LauncherError {
    #[error("the launcher is not initialized yet")]
    NotInitialized,
    #[error("authentication error: {0}")]
    Auth(#[from] tcc_auth::AuthError),
    #[error("{0}")]
    Other(String),
}
