use crate::theme::prelude::*;
use crate::theme::text_input::TextInputStyle;
use crate::theme::text_input::NAME;

/// Creates `iced::widget::text_input::TextInput` widget
///
/// ![text_input][text_input]
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
        .style(theme::TextInput::Custom(Box::new(TextInputStyle)))
}
