use crate::theme::prelude::*;
use crate::theme::toggler::TogglerStyle;

use embed_doc_image::embed_doc_image;

/// Creates `iced::widget::toggler::Toggler` widget
///
/// ![toggler][toggler]
#[embed_doc_image("toggler", "assets/doc/toggler.png")]
pub fn toggler<'a, Message>(
    label: impl Into<Option<String>>,
    is_checked: bool,
    f: impl Fn(bool) -> Message + 'a,
) -> widget::Toggler<'a, Message> {
    widget::toggler(label, is_checked, f)
        .spacing(8)
        .text_size(THEME.global.text.size)
        .style(theme::Toggler::Custom(Box::new(TogglerStyle)))
}
