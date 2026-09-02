//! Avatar component

use freya::prelude::*;
use tcc_auth::MinecraftAccount;

#[derive(PartialEq)]
pub struct Avatar {
    account_id: String,
    width: Size,
    height: Size,
}

impl Avatar {
    pub fn new(account_id: impl Into<String>) -> Self {
        Self {
            account_id: account_id.into(),
            width: Size::px(32.),
            height: Size::px(32.),
        }
    }

    pub fn width(mut self, width: Size) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: Size) -> Self {
        self.height = height;
        self
    }
}

impl Component for Avatar {
    fn render(&self) -> impl IntoElement {
        rect()
            .width(self.width.clone())
            .height(self.height.clone())
            .corner_radius(CornerRadius::new_all(50.))
            .background(colors::brand())
            .child(
                label()
                    .text(self.account_id[..2].to_uppercase())
                    .font_size(14.)
                    .font_weight(FontWeight::BOLD)
                    .color(colors::fg_on_brand()),
            )
    }
}

mod colors {
    use freya::prelude::*;
    
    pub fn brand() -> Color {
        Color::from_rgb(88, 101, 242)
    }
    
    pub fn fg_on_brand() -> Color {
        Color::WHITE
    }
}