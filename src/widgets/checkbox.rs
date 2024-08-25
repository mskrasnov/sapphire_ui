use crate::theme::checkbox::CheckboxStyle;
use crate::theme::prelude::*;

pub fn checkbox<'a, Message>(
    label: impl Into<String>,
    is_checked: bool,
) -> widget::Checkbox<'a, Message> {
    widget::checkbox(label, is_checked)
        .spacing(8)
        .style(theme::Checkbox::Custom(Box::new(CheckboxStyle)))
}
