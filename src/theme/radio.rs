//! Styles for `iced::widget::radio::Radio` widget

use super::prelude::*;

pub const NAME: &str = "radio";

#[derive(Clone, Copy)]
pub struct RadioStyle;

impl widget::radio::StyleSheet for RadioStyle {
    type Style = Theme;

    fn active(&self, _style: &Self::Style, is_selected: bool) -> widget::radio::Appearance {
        widget::radio::Appearance {
            background: if is_selected {
                THEME.global.border.regular_colored
            } else {
                THEME.global.primary_fill_color.regular
            }
            .to_background(),
            dot_color: Color::WHITE,
            border_width: THEME.global.border.width,
            border_color: THEME.global.border.regular_grayscale.to_color(),
            text_color: Some(THEME.global.text.default.to_color()),
        }
    }

    fn hovered(&self, _style: &Self::Style, is_selected: bool) -> widget::radio::Appearance {
        widget::radio::Appearance {
            background: if is_selected {
                THEME.global.border.hovered_colored
            } else {
                THEME.global.primary_fill_color.hovered
            }
            .to_background(),
            dot_color: Color::WHITE,
            border_width: THEME.global.border.width,
            border_color: if is_selected {
                THEME.global.border.hovered_colored
            } else {
                THEME.global.border.hovered_grayscale
            }
            .to_color(),
            text_color: Some(THEME.global.text.hover.to_color()),
        }
    }
}
