//! Onboarding views module

pub mod account;
pub mod bundles;
pub mod downloading;
pub mod language;
pub mod migration;
pub mod preferences;
pub mod selection;
pub mod terms;
pub mod test_support;
pub mod welcome;

pub use account::OnboardingAccount;
pub use bundles::OnboardingBundles;
pub use downloading::OnboardingDownloading;
pub use language::OnboardingLanguage;
pub use migration::OnboardingMigration;
pub use preferences::OnboardingPreferences;
pub use selection::OnboardingSelection;
pub use terms::OnboardingTerms;
pub use welcome::OnboardingWelcome;

use freya::prelude::*;
use crate::components::{Button, Icon, IconType};
use crate::routes::Route;
use crate::theme::colors;

pub fn step_heading(title: &str, description: &str) -> impl IntoElement {
    rect()
        .vertical()
        .spacing(8.)
        .child(
            label()
                .text(title.to_string())
                .font_size(28.)
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

pub fn onboarding_illustration(icon_type: IconType) -> impl IntoElement {
    rect()
        .width(Size::px(120.))
        .height(Size::px(120.))
        .child(
            Icon::new(icon_type)
                .size(120.)
                .color(colors::brand()),
        )
}

pub fn onboarding_page(
    illustration: impl IntoElement,
    content: impl IntoElement,
    nav: impl IntoElement,
) -> impl IntoElement {
    rect()
        .vertical()
        .width(Size::fill())
        .height(Size::fill())
        .spacing(32.)
        .padding(Gaps::new_all(48.))
        .cross_align(Alignment::Center)
        .child(illustration)
        .child(content)
        .child(nav)
}

pub fn onboarding_nav(
    back: Option<Route>,
    next: Route,
    can_continue: bool,
) -> impl IntoElement {
    use crate::hooks::use_dispatch;
    use freya::router::RouterContext;
    
    let dispatch = use_dispatch();
    
    rect()
        .horizontal()
        .width(Size::fill())
        .main_align(Alignment::SpaceBetween)
        .child(
            back.map(|route| {
                Button::new()
                    .ghost()
                    .on_press(move |_| {
                        let _ = RouterContext::get().push(route);
                    })
                    .child(Icon::new(IconType::ChevronLeft).size(16.))
                    .text("Back")
            })
        )
        .child(
            Button::new()
                .primary()
                .enabled(can_continue)
                .on_press(move |_| {
                    let _ = RouterContext::get().push(next);
                })
                .text("Continue")
                .child(Icon::new(IconType::ChevronRight).size(16.))
        )
}