//! Cluster shell layout

use freya::prelude::*;
use freya::router::prelude::Outlet;

#[derive(PartialEq)]
pub struct ClusterShell;

impl Component for ClusterShell {
    fn render(&self) -> impl IntoElement {
        Outlet::<crate::routes::Route>::new()
    }
}
