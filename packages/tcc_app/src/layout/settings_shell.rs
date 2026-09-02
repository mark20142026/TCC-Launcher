//! Settings shell layout

use freya::prelude::*;
use freya::router::prelude::Outlet;

#[derive(PartialEq)]
pub struct SettingsShell;

impl Component for SettingsShell {
    fn render(&self) -> impl IntoElement {
        Outlet::<crate::routes::Route>::new()
    }
}
