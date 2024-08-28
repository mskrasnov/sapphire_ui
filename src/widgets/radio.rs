use crate::theme::prelude::*;
use crate::theme::radio::RadioStyle;

use embed_doc_image::embed_doc_image;

/// Creates `iced::widget::radio::Radio` widget
///
/// ![radio][radio]
#[embed_doc_image("radio", "assets/doc/radio.png")]
pub fn radio<Message, V>(
    label: impl Into<String>,
    value: V,
    selected: Option<V>,
    on_click: impl FnOnce(V) -> Message,
) -> widget::Radio<Message>
where
    Message: Clone,
    V: Copy + Eq,
{
    widget::radio(label, value, selected, on_click)
        .spacing(8)
        .text_size(THEME.global.text.size)
        .size(20)
        .style(theme::Radio::Custom(Box::new(RadioStyle)))
}
