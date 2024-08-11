use crate::theme::prelude::*;
use crate::theme::text::*;

use embed_doc_image::embed_doc_image;

/// Creates the `iced::widget::text::Text` widget
///
/// ![text][text]
#[embed_doc_image("text", "assets/doc/text.png")]
pub fn text<'a, L: ToString>(label: L) -> widget::Text<'a> {
    widget::text(label)
        .variant(TextVariant::Regular)
        .size(Pixels(THEME.global.text.size))
}

/// Creates the big `iced::widget::text::Text` widget
pub fn title_text<'a, L: ToString>(label: L) -> widget::Text<'a> {
    widget::text(label)
        .variant(TextVariant::Regular)
        .size(Pixels(THEME.global.text.size * 2.))
        // .line_height(Pixels(3.))
}

/// Creates the small `iced::widget::text::Text` widget
///
/// ![text_small][text_small]
#[embed_doc_image("text_small", "assets/doc/text_small.png")]
pub fn small_text<'a, L: ToString>(label: L) -> widget::Text<'a> {
    widget::text(label)
        .variant(TextVariant::Dimmed)
        .size(Pixels(THEME.global.text.size - 2.))
}
