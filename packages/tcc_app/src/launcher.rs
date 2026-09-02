//! Launcher module
//!
//! Owns the global [`LauncherCore`] so that queries and actions can reach it
//! without prop drilling. Initialize once at startup with [`init_launcher`].

use std::sync::OnceLock;

use tcc_core::{LauncherCore, LauncherError};

static CORE: OnceLock<LauncherCore> = OnceLock::new();

/// Initializes the global launcher core (auth store, event bus, networking).
///
/// Safe to call more than once; the first completed initialization wins and
/// later calls return the existing core.
pub async fn init_launcher() -> anyhow::Result<&'static LauncherCore> {
    if let Some(core) = CORE.get() {
        return Ok(core);
    }

    let core = LauncherCore::new().await?;
    let _ = CORE.set(core);
    Ok(CORE.get().expect("core was just stored"))
}

/// Returns the global launcher core, or an error if it has not been
/// initialized yet.
pub fn state() -> Result<&'static LauncherCore, LauncherError> {
    CORE.get().ok_or(LauncherError::NotInitialized)
}
