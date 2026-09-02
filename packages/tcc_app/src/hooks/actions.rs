//! Actions for TCC Launcher

use crate::routes::Route;

/// App-wide commands shared through a root context.
#[derive(Clone, Copy, Default)]
pub struct Actions;

impl Actions {
    /// Kept for call-site compatibility; `Actions` is a unit struct.
    pub fn new() -> Self {
        Self
    }

    /// Pushes a route. Must be called from inside the router tree.
    pub fn navigate(&self, route: Route) {
        let _ = freya::router::RouterContext::get().push(route);
    }

    pub fn close_account_switcher(&self) {
        crate::hooks::set_account_switcher_open(false);
    }
}

pub struct NotificationBuilder {
    title: String,
    body: String,
}

impl NotificationBuilder {
    pub fn new(title: impl Into<String>, body: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            body: body.into(),
        }
    }
}

pub struct PumpSignal;
