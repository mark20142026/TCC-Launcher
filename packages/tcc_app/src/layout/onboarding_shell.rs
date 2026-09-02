//! Onboarding shell layout

use freya::prelude::*;
use freya::router::prelude::Outlet;

#[derive(PartialEq)]
pub struct OnboardingShell;

impl Component for OnboardingShell {
    fn render(&self) -> impl IntoElement {
        Outlet::<crate::routes::Route>::new()
    }
}
