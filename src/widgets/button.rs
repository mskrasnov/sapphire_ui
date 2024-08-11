use crate::theme::button::ButtonStyle;
use crate::theme::button::NAME;
use crate::theme::prelude::*;

use embed_doc_image::embed_doc_image;

/// Creates `iced::widget::button::Button` widget
///
/// ![button][button]
#[embed_doc_image("button", "assets/doc/button.png")]
pub fn button<'a, B>(content: impl Into<Element<'a, B>>) -> widget::Button<'a, B> {
    widget::button(content)
        .padding([
            THEME.widget.get(NAME).unwrap().padding.unwrap().0,
            THEME.widget.get(NAME).unwrap().padding.unwrap().1,
        ])
        .style(theme::Button::Custom(Box::new(ButtonStyle)))
}
