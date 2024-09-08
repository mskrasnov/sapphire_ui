//! Styles for `iced::widget::toggler::Toggler` widget

use super::prelude::*;

pub const NAME: &str = "toggler";

#[derive(Clone, Copy)]
pub struct TogglerStyle;

impl widget::toggler::StyleSheet for TogglerStyle {
    type Style = Theme;

    fn active(&self, _style: &Self::Style, is_active: bool) -> widget::toggler::Appearance {
        widget::toggler::Appearance {
            background: if is_active {
                THEME.global.border.regular_colored.to_color()
            } else {
                THEME.global.primary_fill_color.regular.to_color()
            },
            background_border_width: THEME.global.border.width,
            background_border_color: THEME.global.border.regular_grayscale.to_color(),
            foreground: Color::WHITE,
            foreground_border_width: THEME.global.border.width,
            foreground_border_color: THEME.global.border.regular_grayscale.to_color(),
        }
    }

    fn hovered(&self, _style: &Self::Style, is_active: bool) -> widget::toggler::Appearance {
        widget::toggler::Appearance {
            background: if is_active {
                THEME.global.border.hovered_colored.to_color()
            } else {
                THEME.global.primary_fill_color.hovered.to_color()
            },
            background_border_width: THEME.global.border.width,
            background_border_color: THEME.global.border.regular_grayscale.to_color(),
            foreground: Color::WHITE,
            foreground_border_width: THEME.global.border.width,
            foreground_border_color: THEME.global.border.regular_grayscale.to_color(),
        }
    }
}
