//! Styles for `iced::widget::scrollable::Scrollable` widget

use super::prelude::*;

pub const NAME: &str = "scrollable";

#[derive(Clone, Copy)]
pub struct ScrollableStyle;

impl widget::scrollable::StyleSheet for ScrollableStyle {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> widget::scrollable::Appearance {
        let border = &THEME.global.border;
        widget::scrollable::Appearance {
            scrollbar: widget::scrollable::Scrollbar {
                background: Some(THEME.global.primary_fill_color.regular.to_background()),
                border: Border {
                    color: border.regular_grayscale.to_color(),
                    width: border.width,
                    radius: Radius::from(border.radius),
                },
                scroller: widget::scrollable::Scroller {
                    color: border.regular_grayscale.to_color(),
                    border: Border {
                        color: border.regular_grayscale.to_color(),
                        width: border.width,
                        radius: Radius::from(border.radius),
                    },
                },
            },
            container: iced::widget::container::Appearance {
                background: Some(THEME.global.container_background.to_background()),
                text_color: Some(THEME.global.text.default.to_color()),
                ..Default::default()
            },
            gap: None,
        }
    }

    fn hovered(
        &self,
        _style: &Self::Style,
        is_mouse_over_scrollbar: bool,
    ) -> widget::scrollable::Appearance {
        let border = &THEME.global.border;
        widget::scrollable::Appearance {
            scrollbar: widget::scrollable::Scrollbar {
                background: Some(THEME.global.primary_fill_color.regular.to_background()),
                border: Border {
                    color: if is_mouse_over_scrollbar {
                        border.hovered_grayscale
                    } else {
                        border.regular_grayscale
                    }
                    .to_color(),
                    width: border.width,
                    radius: Radius::from(border.radius),
                },
                scroller: widget::scrollable::Scroller {
                    color: if is_mouse_over_scrollbar {
                        border.hovered_grayscale
                    } else {
                        border.regular_grayscale
                    }
                    .to_color(),
                    border: Border {
                        color: border.regular_grayscale.to_color(),
                        width: border.width,
                        radius: Radius::from(border.radius),
                    },
                },
            },
            container: iced::widget::container::Appearance {
                background: Some(THEME.global.container_background.to_background()),
                text_color: Some(THEME.global.text.default.to_color()),
                ..Default::default()
            },
            gap: None,
        }
    }
}
