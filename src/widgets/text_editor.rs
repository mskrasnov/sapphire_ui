use crate::theme::prelude::*;
use crate::theme::text_editor::TextEditorStyle;

use iced::advanced::text::highlighter::PlainText;
use iced::advanced::text::Renderer;
use iced::widget::text_editor::Content;
use iced::Font;

/// Creates `iced::widget::text_editor::TextEditor` widget
pub fn text_editor<Message, Theme, R>(
    content: &Content<R>,
) -> widget::TextEditor<'_, PlainText, Message, Theme, R>
where
    Message: Clone,
    Theme: widget::text_editor::StyleSheet,
    R: Renderer,
    <Theme as iced::widget::text_editor::StyleSheet>::Style: From<iced::theme::TextEditor>,
    <R as iced::advanced::text::Renderer>::Font: From<iced::Font>,
{
    widget::text_editor(content)
        .font(Font::with_name(&THEME.global.text.font))
        .style(theme::TextEditor::Custom(Box::new(TextEditorStyle)))
}
