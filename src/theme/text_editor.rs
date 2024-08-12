//! Styles for `iced::widget::text_editor::TextEditor` widget

use super::prelude::*;

pub const NAME: &str = "text_editor";

#[derive(Clone, Copy)]
pub struct TextEditorStyle;

impl widget::text_editor::StyleSheet for TextEditorStyle {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> widget::text_editor::Appearance {
        let background = THEME.widget.get(NAME).unwrap().background;
        let border = &THEME.global.border;

        widget::text_editor::Appearance {
            background: background
                .unwrap_or(THEME.global.secondary_fill_color)
                .to_background(),
            border: Border {
                color: border.regular_grayscale.to_color(),
                width: border.width,
                radius: Radius::from(border.radius),
            },
        }
    }

    fn focused(&self, _style: &Self::Style) -> widget::text_editor::Appearance {
        self.active(_style)
    }

    fn placeholder_color(&self, _style: &Self::Style) -> Color {
        THEME.global.text.disabled.to_color()
    }

    fn value_color(&self, _style: &Self::Style) -> Color {
        THEME.global.text.default.to_color()
    }

    fn selection_color(&self, _style: &Self::Style) -> Color {
        THEME.global.primary_fill_color.pressed.to_color()
    }

    fn disabled_color(&self, _style: &Self::Style) -> Color {
        THEME.global.text.disabled.to_color()
    }

    fn hovered(&self, _style: &Self::Style) -> widget::text_editor::Appearance {
        self.active(_style)
    }

    fn disabled(&self, _style: &Self::Style) -> widget::text_editor::Appearance {
        self.active(_style)
    }
}
