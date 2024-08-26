//! Styles for `iced::widget::text_input::TextInput` widget

use super::prelude::*;

pub const NAME: &str = "text_input";

#[derive(Clone, Copy)]
pub struct TextInputStyle;

impl widget::text_input::StyleSheet for TextInputStyle {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> widget::text_input::Appearance {
        let background = THEME.widget.get(NAME).unwrap().background;
        let border = &THEME.global.border;
        let text = &THEME.global.text;

        widget::text_input::Appearance {
            background: background
                .unwrap_or(THEME.global.secondary_fill_color)
                .to_background(),
            border: Border {
                color: border.regular_grayscale.to_color(),
                width: border.width,
                radius: Radius::from(border.radius),
            },
            icon_color: text.default.to_color(),
        }
    }

    fn focused(&self, _style: &Self::Style) -> widget::text_input::Appearance {
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

    fn hovered(&self, _style: &Self::Style) -> widget::text_input::Appearance {
        let background = THEME.widget.get(NAME).unwrap().background;
        let border = &THEME.global.border;
        let text = &THEME.global.text;

        widget::text_input::Appearance {
            background: background
                .unwrap_or(THEME.global.secondary_fill_color)
                .to_background(),
            border: Border {
                color: border.hovered_colored.to_color(),
                width: border.width,
                radius: Radius::from(border.radius),
            },
            icon_color: text.default.to_color(),
        }
    }

    fn disabled(&self, _style: &Self::Style) -> widget::text_input::Appearance {
        let background = THEME.widget.get(NAME).unwrap().background;
        let border = &THEME.global.border;
        let text = &THEME.global.text;

        widget::text_input::Appearance {
            background: background
                .unwrap_or(THEME.global.primary_fill_color.disabled)
                .to_background(),
            border: Border {
                color: border.regular_grayscale.to_color(),
                width: border.width,
                radius: Radius::from(border.radius),
            },
            icon_color: text.default.to_color(),
        }
    }
}
