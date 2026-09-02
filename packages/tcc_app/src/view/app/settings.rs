//! Settings views

use freya::prelude::*;
use crate::theme::colors;

#[derive(PartialEq)]
pub struct SettingsApis;

impl Component for SettingsApis {
    fn render(&self) -> impl IntoElement {
        settings_page("APIs", "Configure API endpoints")
    }
}

#[derive(PartialEq)]
pub struct SettingsAppearance;

impl Component for SettingsAppearance {
    fn render(&self) -> impl IntoElement {
        settings_page("Appearance", "Customize the look and feel")
    }
}

#[derive(PartialEq)]
pub struct SettingsChangelog;

impl Component for SettingsChangelog {
    fn render(&self) -> impl IntoElement {
        settings_page("Changelog", "View version history")
    }
}

#[derive(PartialEq)]
pub struct SettingsDeveloper;

impl Component for SettingsDeveloper {
    fn render(&self) -> impl IntoElement {
        settings_page("Developer", "Developer options")
    }
}

#[derive(PartialEq)]
pub struct SettingsJava;

impl Component for SettingsJava {
    fn render(&self) -> impl IntoElement {
        settings_page("Java", "Configure Java runtime")
    }
}

#[derive(PartialEq)]
pub struct SettingsLanguage;

impl Component for SettingsLanguage {
    fn render(&self) -> impl IntoElement {
        settings_page("Language", "Select language")
    }
}

#[derive(PartialEq)]
pub struct SettingsLauncher;

impl Component for SettingsLauncher {
    fn render(&self) -> impl IntoElement {
        rect()
            .width(Size::fill())
            .height(Size::fill())
            .padding(Gaps::new_all(24.))
            .spacing(12.)
            .child(
                label()
                    .text("Launcher")
                    .font_size(24.)
                    .font_weight(FontWeight::BOLD)
                    .color(colors::fg_primary()),
            )
            .child(
                label()
                    .text("Launcher settings")
                    .font_size(16.)
                    .color(colors::fg_secondary()),
            )
            .child(
                label()
                    .text(format!(
                        "Current version: {}",
                        crate::constants::APP_VERSION
                    ))
                    .font_size(14.)
                    .color(colors::fg_primary()),
            )
            .child(
                crate::components::Button::new()
                    .secondary()
                    .on_press(|_| {
                        // The dialog that reports the result runs from the
                        // worker thread, so no state plumbing is needed here.
                        std::thread::spawn(crate::updater::check_and_install_interactive);
                    })
                    .text("Check for Updates"),
            )
    }
}

#[derive(PartialEq)]
pub struct SettingsMinecraft;

impl Component for SettingsMinecraft {
    fn render(&self) -> impl IntoElement {
        settings_page("Minecraft", "Minecraft settings")
    }
}

#[derive(PartialEq)]
pub struct SettingsStorage;

impl Component for SettingsStorage {
    fn render(&self) -> impl IntoElement {
        settings_page("Storage", "Manage storage locations")
    }
}

fn settings_page(title: &str, description: &str) -> impl IntoElement {
    rect()
        .width(Size::fill())
        .height(Size::fill())
        .padding(Gaps::new_all(24.))
        .child(
            label()
                .text(title.to_string())
                .font_size(24.)
                .font_weight(FontWeight::BOLD)
                .color(colors::fg_primary()),
        )
        .child(
            label()
                .text(description.to_string())
                .font_size(16.)
                .color(colors::fg_secondary()),
        )
}
