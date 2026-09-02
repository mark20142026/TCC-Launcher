//! TCC Launcher Freya App library

use freya::prelude::*;

pub mod assets;
pub mod components;
pub mod constants;
pub mod events;
pub mod hooks;
pub mod install;
pub mod launcher;
pub mod layout;
pub mod notifications;
pub mod platform;
pub mod routes;
pub mod state;
pub mod theme;
pub mod transfer;
pub mod ui;
pub mod updater;
pub mod utils;
pub mod view;

/// Runs the launcher: initializes the core, kicks off the auto-update check,
/// then enters the UI event loop.
pub fn run(_devtools: bool) {
    {
        match tokio::runtime::Runtime::new() {
            Ok(rt) => {
                if let Err(e) = rt.block_on(async { crate::launcher::init_launcher().await }) {
                    tracing::error!("launcher core failed to initialize: {e:#}");
                }
            }
            Err(e) => tracing::error!("failed to create async runtime: {e}"),
        }
    }

    std::thread::spawn(crate::updater::auto_check_background);

    let window = WindowConfig::new(app)
        .with_title("TCC Launcher")
        .with_size(1200.0, 800.0);
    let config = LaunchConfig::new().with_window(window);
    launch(config);
}

fn app() -> Element {
    use crate::hooks::use_provide_actions;

    let actions = crate::hooks::Actions::new();
    use_provide_actions(&actions);

    crate::router().into_element()
}
