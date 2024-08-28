use crate::theme::checkbox::CheckboxStyle;
use crate::theme::prelude::*;

use embed_doc_image::embed_doc_image;

/// Creates `iced::widget::checkbox::Checkbox` widget
///
/// ![checkbox][checkbox]
#[embed_doc_image("checkbox", "assets/doc/checkbox.png")]
pub fn checkbox<'a, Message>(
    label: impl Into<String>,
    is_checked: bool,
) -> widget::Checkbox<'a, Message> {
    widget::checkbox(label, is_checked)
        .spacing(8)
        .text_size(THEME.global.text.size)
        .style(theme::Checkbox::Custom(Box::new(CheckboxStyle)))
}
