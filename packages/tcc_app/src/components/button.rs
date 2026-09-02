//! Button component

use freya::prelude::*;
use crate::theme::colors;

#[derive(PartialEq, Clone, Copy)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Ghost,
    Danger,
}

#[derive(PartialEq, Clone, Copy)]
pub enum ButtonSize {
    Small,
    Medium,
    Large,
}

#[derive(PartialEq)]
pub struct Button {
    variant: ButtonVariant,
    size: ButtonSize,
    enabled: bool,
    on_press: Option<EventHandler<Event<PressEventData>>>,
    children: Vec<Element>,
    text: Option<String>,
}

impl Button {
    pub fn new() -> Self {
        Self {
            variant: ButtonVariant::Primary,
            size: ButtonSize::Medium,
            enabled: true,
            on_press: None,
            children: Vec::new(),
            text: None,
        }
    }

    pub fn primary(mut self) -> Self {
        self.variant = ButtonVariant::Primary;
        self
    }

    pub fn secondary(mut self) -> Self {
        self.variant = ButtonVariant::Secondary;
        self
    }

    pub fn ghost(mut self) -> Self {
        self.variant = ButtonVariant::Ghost;
        self
    }

    pub fn danger(mut self) -> Self {
        self.variant = ButtonVariant::Danger;
        self
    }

    pub fn small(mut self) -> Self {
        self.size = ButtonSize::Small;
        self
    }

    pub fn large(mut self) -> Self {
        self.size = ButtonSize::Large;
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn on_press(mut self, handler: impl Fn(Event<PressEventData>) + 'static) -> Self {
        self.on_press = Some(EventHandler::new(handler));
        self
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_element());
        self
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }
}

impl Default for Button {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for Button {
    fn render(&self) -> impl IntoElement {
        let (bg_color, fg_color, border_color) = match self.variant {
            ButtonVariant::Primary => (colors::brand(), colors::fg_on_brand(), colors::brand()),
            ButtonVariant::Secondary => (colors::component_bg(), colors::fg_primary(), colors::component_border()),
            ButtonVariant::Ghost => (Color::TRANSPARENT, colors::fg_primary(), Color::TRANSPARENT),
            ButtonVariant::Danger => (colors::danger(), colors::fg_on_brand(), colors::danger()),
        };

        let (padding_x, padding_y, font_size) = match self.size {
            ButtonSize::Small => (12., 6., 12.),
            ButtonSize::Medium => (16., 10., 14.),
            ButtonSize::Large => (24., 14., 16.),
        };

        let mut btn = rect()
            .horizontal()
            .cross_align(Alignment::Center)
            .spacing(8.)
            .padding(Gaps::new(padding_y, padding_x, padding_y, padding_x))
            .corner_radius(CornerRadius::new_all(8.))
            .background(if self.enabled { bg_color } else { bg_color.with_a(128) })
            .border(
                Border::new()
                    .fill(border_color)
                    .width(1.)
                    .alignment(BorderAlignment::Inner),
            );

        if let Some(handler) = &self.on_press {
            let handler = handler.clone();
            btn = btn.on_press(move |e| handler.call(e));
        }

        btn = btn.cursor(if self.enabled { CursorIcon::Pointer } else { CursorIcon::Default });

        // Add children
        for child in &self.children {
            btn = btn.child(child.clone());
        }

        // Add text if provided
        if let Some(text) = &self.text {
            btn = btn.child(
                label()
                    .text(text.clone())
                    .font_size(font_size)
                    .font_weight(FontWeight::MEDIUM)
                    .color(if self.enabled { fg_color } else { fg_color.with_a(128) }),
            );
        }

        btn.into_element()
    }
}