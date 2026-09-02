//! State management for TCC Launcher

use freya::prelude::*;
use tcc_auth::AuthService;
use tcc_events::EventBus;
use tcc_core::LauncherCore;
use std::sync::Arc;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AppChannel {
    Launcher,
    Settings,
    Notifications,
    AccountSwitcher,
    Game,
    Installs,
    MicrosoftLogin,
}

pub struct LauncherInit {
    pub core: Option<LauncherCore>,
    pub auth: Option<AuthService>,
    pub events: Option<EventBus>,
}

impl LauncherInit {
    pub fn new() -> Self {
        Self {
            core: None,
            auth: None,
            events: None,
        }
    }
}

impl Default for LauncherInit {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
pub struct SettingsState {
    // Settings state fields
}

#[derive(Clone, Default)]
pub struct GameState {
    // Game state fields
}

#[derive(Clone, Default)]
pub struct InstallState {
    // Install state fields
}

#[derive(Clone, Default)]
pub struct LoginProgress {
    pub current: u64,
    pub total: u64,
    pub label: String,
}