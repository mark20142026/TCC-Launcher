//! Overlay popup component

use freya::prelude::*;

#[derive(PartialEq)]
pub struct OverlayPopup {
    position: Option<Position>,
    on_close: Option<Box<dyn Fn()>>,
    child: Element,
}

impl OverlayPopup {
    pub fn new() -> Self {
        Self {
            position: None,
            on_close: None,
            child: rect().into_element(),
        }
    }

    pub fn position(mut self, position: Position) -> Self {
        self.position = Some(position);
        self
    }

    pub fn on_close(mut self, handler: impl Fn() + 'static) -> Self {
        self.on_close = Some(Box::new(handler));
        self
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.child = child.into_element();
        self
    }
}

impl Default for OverlayPopup {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for OverlayPopup {
    fn render(&self) -> impl IntoElement {
        let on_close = self.on_close.clone();

        let mut overlay = rect()
            .width(Size::fill())
            .height(Size::fill())
            .background(Color::from_argb(128, 0, 0, 0))
            .on_press(move |_| {
                if let Some(handler) = &on_close {
                    handler();
                }
            });

        if let Some(position) = self.position.clone() {
            overlay = overlay.position(position);
        }

        overlay.child(self.child.clone())
    }
}
