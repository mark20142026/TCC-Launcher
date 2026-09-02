//! Dropdown component
//!
//! Static selection display: renders every item and reports clicks through
//! `on_select`. Interactive open/close state can be layered on top once the
//! popup API is settled.

use freya::prelude::*;

use super::{Icon, IconType};

pub struct Dropdown<T> {
    items: Vec<(String, T)>,
    selected: Option<T>,
    on_select: Option<Box<dyn Fn(T)>>,
    placeholder: String,
}

impl<T> Dropdown<T> {
    pub fn new(items: Vec<(String, T)>) -> Self {
        Self {
            items,
            selected: None,
            on_select: None,
            placeholder: "Select...".to_string(),
        }
    }

    pub fn selected(mut self, selected: Option<T>) -> Self {
        self.selected = selected;
        self
    }

    pub fn on_select(mut self, handler: impl Fn(T) + 'static) -> Self {
        self.on_select = Some(Box::new(handler));
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }
}

impl<T> Default for Dropdown<T> {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

impl<T: PartialEq> PartialEq for Dropdown<T> {
    fn eq(&self, other: &Self) -> bool {
        self.selected == other.selected
            && self.placeholder == other.placeholder
            && self.items.len() == other.items.len()
            && self
                .items
                .iter()
                .zip(other.items.iter())
                .all(|(a, b)| a.0 == b.0 && a.1 == b.1)
    }
}

impl<T: PartialEq + Clone + 'static> Component for Dropdown<T> {
    fn render(&self) -> impl IntoElement {
        let selected_text = self
            .selected
            .as_ref()
            .and_then(|s| self.items.iter().find(|(_, v)| v == s).map(|(k, _)| k.clone()))
            .unwrap_or_else(|| self.placeholder.clone());

        let mut dropdown = rect()
            .vertical()
            .width(Size::fill())
            .spacing(4.)
            .padding(Gaps::new_symmetric(12., 8.))
            .corner_radius(CornerRadius::new_all(8.))
            .background(crate::theme::colors::component_bg())
            .border(
                Border::new()
                    .fill(crate::theme::colors::component_border())
                    .width(1.)
                    .alignment(BorderAlignment::Inner),
            )
            .child(
                rect()
                    .horizontal()
                    .cross_align(Alignment::Center)
                    .spacing(8.)
                    .child(
                        label()
                            .text(selected_text)
                            .font_size(14.)
                            .color(crate::theme::colors::fg_primary()),
                    )
                    .child(
                        Icon::new(IconType::ChevronDown)
                            .size(16.)
                            .color(crate::theme::colors::fg_secondary()),
                    ),
            );

        for (item_label, item_value) in &self.items {
            let is_selected = self.selected.as_ref().is_some_and(|s| s == item_value);
            let item_value = item_value.clone();
            let on_select = self.on_select.clone();

            dropdown = dropdown.child(
                rect()
                    .horizontal()
                    .width(Size::fill())
                    .cross_align(Alignment::Center)
                    .spacing(8.)
                    .padding(Gaps::new_symmetric(12., 8.))
                    .background(if is_selected {
                        crate::theme::colors::ghost_overlay_hover()
                    } else {
                        crate::theme::colors::component_bg()
                    })
                    .on_press(move |_| {
                        if let Some(handler) = &on_select {
                            handler(item_value.clone());
                        }
                    })
                    .child(
                        label()
                            .text(item_label.clone())
                            .font_size(14.)
                            .color(crate::theme::colors::fg_primary()),
                    ),
            );
        }

        dropdown.into_element()
    }
}
