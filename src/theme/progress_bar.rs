//! Styles for `iced::widget::progress_bar::ProgressBar` widget

use super::prelude::*;

pub const NAME: &str = "progress_bar";

#[derive(Clone, Copy)]
pub struct ProgressBarStyle;

impl widget::progress_bar::StyleSheet for ProgressBarStyle {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> widget::progress_bar::Appearance {
        widget::progress_bar::Appearance {
            background: THEME.global.secondary_fill_color.to_background(),
            bar: THEME.global.border.regular_colored.to_background(),
            border_radius: Radius::from(THEME.global.border.radius),
        }
    }
}
