//! Hooks for TCC Launcher

mod actions;
mod queries;
mod view_state;

pub use actions::{Actions, NotificationBuilder, PumpSignal};
pub use queries::*;
pub use view_state::{PersistedView, use_view_state};

use std::sync::atomic::{AtomicBool, Ordering};

use crate::notifications::{NotificationCenter, NotificationSnapshot};
use crate::state::{GameState, InstallState, LauncherInit, LoginProgress, SettingsState};
use freya::prelude::*;

/// Whether the account switcher popup is open. A plain global for now; the
/// popup is opened from the shell once that part is wired up.
static ACCOUNT_SWITCHER_OPEN: AtomicBool = AtomicBool::new(false);

pub fn set_account_switcher_open(open: bool) {
    ACCOUNT_SWITCHER_OPEN.store(open, Ordering::Relaxed);
}

/// Publishes the actions handle so components can reach it without prop
/// drilling. Provided once, at the root.
pub fn use_provide_actions(actions: &Actions) {
    let actions = actions.clone();
    use_provide_root_context(move || actions.clone());
}

pub fn use_dispatch() -> Actions {
    consume_root_context::<Actions>()
}

pub fn use_launcher() -> LauncherInit {
    LauncherInit::new()
}

pub fn use_settings_snapshot() -> SettingsState {
    SettingsState::default()
}

pub fn use_notifications_snapshot() -> NotificationSnapshot {
    NotificationCenter::new().snapshot(&(), false, ())
}

pub fn use_account_switcher_open() -> bool {
    ACCOUNT_SWITCHER_OPEN.load(Ordering::Relaxed)
}

pub fn use_game_snapshot() -> GameState {
    GameState::default()
}

pub fn use_installs_snapshot() -> InstallState {
    InstallState::default()
}

pub fn use_offline_login_status() -> Option<LoginProgress> {
    None // No Microsoft login in TCC
}
