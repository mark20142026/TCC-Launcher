//! Theme colors

use freya::prelude::*;

pub fn brand() -> Color {
    Color::from_rgb(88, 101, 242)
}

pub fn fg_primary() -> Color {
    Color::from_rgb(255, 255, 255)
}

pub fn fg_secondary() -> Color {
    Color::from_rgb(180, 180, 180)
}

pub fn fg_on_brand() -> Color {
    Color::WHITE
}

pub fn component_bg() -> Color {
    Color::from_rgb(40, 40, 40)
}

pub fn component_border() -> Color {
    Color::from_rgb(80, 80, 80)
}

pub fn page_elevated() -> Color {
    Color::from_rgb(30, 30, 30)
}

pub fn ghost_overlay_hover() -> Color {
    Color::from_argb(20, 255, 255, 255)
}

pub fn success() -> Color {
    Color::from_rgb(67, 181, 129)
}

pub fn danger() -> Color {
    Color::from_rgb(237, 66, 69)
}

pub fn divider() -> impl IntoElement {
    rect()
        .width(Size::fill())
        .height(Size::px(1.))
        .background(component_border())
}