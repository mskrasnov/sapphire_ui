//! Styles for `iced::widget::slider::Slider` widget

use super::prelude::*;

pub const NAME: &str = "slider";

#[derive(Clone, Copy)]
pub struct SliderStyle;

impl widget::slider::StyleSheet for SliderStyle {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> widget::slider::Appearance {
        widget::slider::Appearance {
            rail: widget::slider::Rail {
                colors: (
                    THEME.global.border.regular_colored.to_color(),
                    THEME.global.secondary_fill_color.to_color(),
                ),
                width: 10.,
                border_radius: Radius::from(THEME.global.border.radius),
            },
            handle: widget::slider::Handle {
                shape: widget::slider::HandleShape::Circle { radius: 10. },
                color: Color::WHITE,
                border_width: 1.,
                border_color: THEME.global.border.regular_grayscale.to_color(),
            },
        }
    }

    fn hovered(&self, _style: &Self::Style) -> widget::slider::Appearance {
        widget::slider::Appearance {
            rail: widget::slider::Rail {
                colors: (
                    THEME.global.border.regular_colored.to_color(),
                    THEME.global.secondary_fill_color.to_color(),
                ),
                width: 10.,
                border_radius: Radius::from(THEME.global.border.radius),
            },
            handle: widget::slider::Handle {
                shape: widget::slider::HandleShape::Circle { radius: 10. },
                color: THEME.global.primary_fill_color.hovered.to_color(),
                border_width: 1.,
                border_color: THEME.global.border.regular_grayscale.to_color(),
            },
        }
    }

    fn dragging(&self, _style: &Self::Style) -> widget::slider::Appearance {
        widget::slider::Appearance {
            rail: widget::slider::Rail {
                colors: (
                    THEME.global.border.regular_colored.to_color(),
                    THEME.global.secondary_fill_color.to_color(),
                ),
                width: 10.,
                border_radius: Radius::from(THEME.global.border.radius),
            },
            handle: widget::slider::Handle {
                shape: widget::slider::HandleShape::Circle { radius: 10. },
                color: THEME.global.primary_fill_color.pressed.to_color(),
                border_width: 1.,
                border_color: THEME.global.border.regular_grayscale.to_color(),
            },
        }
    }
}
