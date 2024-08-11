use crate::theme::prelude::*;
use crate::theme::scrollable::ScrollableStyle;

use embed_doc_image::embed_doc_image;

/// Creates `iced::widget::scrollable::Scrollable` widget
///
/// ![scrollable][scrollable]
#[embed_doc_image("scrollable", "assets/doc/scrollable.png")]
pub fn scrollable<'a, Message, Theme>(
    content: impl Into<Element<'a, Message, Theme>>,
) -> widget::Scrollable<'a, Message, Theme>
where
    Theme: widget::scrollable::StyleSheet,
    <Theme as widget::scrollable::StyleSheet>::Style: From<theme::Scrollable>,
{
    widget::scrollable(content).style(theme::Scrollable::Custom(Box::new(ScrollableStyle)))
}
