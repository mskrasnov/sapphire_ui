//! Styles for `iced::widget::checkbox::CheckBox` widget

use super::prelude::*;

pub const NAME: &str = "checkbox";

#[derive(Clone, Copy)]
pub struct CheckboxStyle;

impl widget::checkbox::StyleSheet for CheckboxStyle {
    type Style = Theme;

    fn active(&self, _style: &Self::Style, is_checked: bool) -> widget::checkbox::Appearance {
        widget::checkbox::Appearance {
            background: if is_checked {
                // THEME.global.primary_fill_color.hovered.to_background()
                THEME.global.border.hovered_colored.to_background()
            } else {
                THEME.global.primary_fill_color.regular.to_background()
            },
            border: Border {
                color: THEME.global.border.regular_grayscale.to_color(),
                radius: Radius::from(THEME.global.border.radius),
                width: THEME.global.border.width,
            },
            icon_color: Color::WHITE,
            text_color: Some(THEME.global.text.default.to_color()),
        }
    }

    fn hovered(&self, _style: &Self::Style, is_checked: bool) -> widget::checkbox::Appearance {
        widget::checkbox::Appearance {
            background: match is_checked {
                true => THEME.global.border.hovered_grayscale.to_background(),
                false => THEME.global.primary_fill_color.hovered.to_background(),
            },
            border: Border {
                color: match is_checked {
                    true => THEME.global.border.hovered_colored.to_color(),
                    false => THEME.global.border.hovered_grayscale.to_color(),
                },
                radius: Radius::from(THEME.global.border.radius),
                width: THEME.global.border.width,
            },
            icon_color: Color::WHITE,
            text_color: Some(THEME.global.text.hover.to_color()),
        }
    }
}
