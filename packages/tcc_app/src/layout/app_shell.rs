//! App shell layout

use freya::prelude::*;
use freya::router::Outlet;
use crate::components::AccountSwitcher;

#[derive(PartialEq)]
pub struct AppShell;

impl Component for AppShell {
    fn render(&self) -> impl IntoElement {
        rect()
            .width(Size::fill())
            .height(Size::fill())
            .child(Outlet::<crate::routes::Route>::new())
            .child(AccountSwitcher)
    }
}
