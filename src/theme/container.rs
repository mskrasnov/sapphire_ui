//! Styles for `iced::widget::container::Container` widget

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
