//! Root layout

use freya::prelude::*;
use freya::router::Outlet;

#[derive(PartialEq)]
pub struct RootLayout;

impl Component for RootLayout {
    fn render(&self) -> impl IntoElement {
        Outlet::<crate::routes::Route>::new()
    }
}
