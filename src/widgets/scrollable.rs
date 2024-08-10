use crate::theme::prelude::*;
use crate::theme::scrollable::ScrollableStyle;

pub fn scrollable<'a, Message, Theme>(
    content: impl Into<Element<'a, Message, Theme>>,
) -> widget::Scrollable<'a, Message, Theme>
where
    Theme: widget::scrollable::StyleSheet,
    <Theme as widget::scrollable::StyleSheet>::Style: From<theme::Scrollable>,
{
    widget::scrollable(content).style(theme::Scrollable::Custom(Box::new(ScrollableStyle)))
}
