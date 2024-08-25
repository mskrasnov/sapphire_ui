//! Styles for `iced::widget::container::Container` widget

use iced::Shadow;
use iced::Vector;

use super::prelude::*;

pub const NAME: &str = "container";

#[derive(Clone, Copy)]
pub struct ContainerStyle;

impl widget::container::StyleSheet for ContainerStyle {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> widget::container::Appearance {
        widget::container::Appearance {
            background: Some(THEME.global.container_background.to_background()),
            text_color: Some(THEME.global.text.default.to_color()),
            ..Default::default()
        }
    }
}

#[derive(Clone, Copy)]
pub struct WidgetGroup;

impl widget::container::StyleSheet for WidgetGroup {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> widget::container::Appearance {
        widget::container::Appearance {
            background: Some(THEME.global.widget_group_background.to_background()),
            text_color: Some(THEME.global.text.default.to_color()),
            shadow: Shadow {
                color: THEME.global.primary_fill_color.regular.to_color(),
                offset: Vector::new(2., 2.),
                blur_radius: 2.,
            },
            border: Border {
                color: THEME.global.border.regular_grayscale.to_color(),
                width: THEME.global.border.width,
                radius: Radius::from(THEME.global.border.radius),
            },
            ..Default::default()
        }
    }
}
