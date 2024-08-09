//! Styles for `iced::widget::button::Button` widget

use super::prelude::*;
use iced::Shadow;
use iced::Vector;

pub const NAME: &str = "button";

#[derive(Clone, Copy)]
pub struct ButtonStyle;

impl widget::button::StyleSheet for ButtonStyle {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> widget::button::Appearance {
        let background = THEME.widget.get(NAME).unwrap().background;
        let border = THEME.widget.get(NAME).unwrap().border.as_ref().unwrap();
        let text = THEME.widget.get(NAME).unwrap().text.as_ref().unwrap();

        widget::button::Appearance {
            background: Some(
                background
                    .unwrap_or(THEME.global.primary_fill_color.regular)
                    .to_background(),
            ),
            shadow: Shadow {
                color: THEME.global.primary_fill_color.regular.to_color(),
                offset: Vector::new(1., 1.),
                blur_radius: 2.,
            },
            border: Border {
                color: border.regular_grayscale.to_color(),
                width: border.width,
                radius: Radius::from(border.radius),
            },
            text_color: text.default.to_color(),
            ..Default::default()
        }
    }

    fn hovered(&self, _style: &Self::Style) -> widget::button::Appearance {
        let text = THEME.widget.get(NAME).unwrap().text.as_ref().unwrap();
        let border = THEME.widget.get(NAME).unwrap().border.as_ref().unwrap();

        widget::button::Appearance {
            background: Some(THEME.global.primary_fill_color.hovered.to_background()),
            border: Border {
                color: border.regular_grayscale.to_color(),
                width: border.width,
                radius: Radius::from(border.radius),
            },
            shadow: Shadow {
                color: THEME.global.primary_fill_color.regular.to_color(),
                offset: Vector::new(1., 1.),
                blur_radius: 2.,
            },
            text_color: text.hover.to_color(),
            ..Default::default()
        }
    }

    fn pressed(&self, _style: &Self::Style) -> widget::button::Appearance {
        let background = THEME.widget.get(NAME).unwrap().colors.as_ref().unwrap();
        let border = THEME.widget.get(NAME).unwrap().border.as_ref().unwrap();
        let text = THEME.widget.get(NAME).unwrap().text.as_ref().unwrap();

        widget::button::Appearance {
            background: Some(
                background
                    .get("background_pressed")
                    .unwrap()
                    .to_background(),
            ),
            border: Border {
                color: border.hovered_colored.to_color(),
                width: border.width,
                radius: Radius::from(border.radius),
            },
            shadow: Shadow {
                color: THEME.global.primary_fill_color.regular.to_color(),
                offset: Vector::new(1., 1.),
                blur_radius: 2.,
            },
            text_color: text.pressed.to_color(),
            ..Default::default()
        }
    }

    fn disabled(&self, _style: &Self::Style) -> widget::button::Appearance {
        let background = THEME.widget.get(NAME).unwrap().colors.as_ref().unwrap();
        let border = THEME.widget.get(NAME).unwrap().border.as_ref().unwrap();
        let text = THEME.widget.get(NAME).unwrap().text.as_ref().unwrap();

        widget::button::Appearance {
            background: Some(
                background
                    .get("background_disabled")
                    .unwrap()
                    .to_background(),
            ),
            border: Border {
                color: border.regular_grayscale.to_color(),
                width: border.width,
                radius: Radius::from(border.radius),
            },
            text_color: text.pressed.to_color(),
            ..Default::default()
        }
    }
}
