use crate::theme::prelude::*;
use crate::theme::text_input::TextInputStyle;
use crate::theme::text_input::NAME;

use embed_doc_image::embed_doc_image;

/// Creates `iced::widget::text_input::TextInput` widget
///
/// ![text_input][text_input]
#[embed_doc_image("text_input", "assets/doc/text_input.png")]
pub fn text_input<'a, P, L, Message>(placeholder: P, label: L) -> widget::TextInput<'a, Message>
where
    P: ToString,
    L: ToString,
    Message: Clone,
{
    widget::text_input(&placeholder.to_string(), &label.to_string())
        .padding([
            THEME.widget.get(NAME).unwrap().padding.unwrap().0,
            THEME.widget.get(NAME).unwrap().padding.unwrap().1,
        ])
        .size(THEME.global.text.size)
        .style(theme::TextInput::Custom(Box::new(TextInputStyle)))
}
