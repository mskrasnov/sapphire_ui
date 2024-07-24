use iced::border::Radius;
use iced::widget::checkbox;
use iced::Background;
use iced::Border;
use iced::Color;

use crate::accent::BorderColorVariant;
use crate::accent::ColorExt;
use crate::accent::PrimaryFillColorVariant;
use crate::common::TEXT_COLOR_DEFAULT;
use crate::common::TEXT_COLOR_HOVER;
use crate::theme::Theme;

static CHECKBOX_DARKER_GRAY: Color = Color::from_rgb(0.11, 0.11, 0.11);
static CHECKBOX_LIGHTER_GRAY: Color = Color::from_rgb(0.12, 0.12, 0.12);

impl checkbox::StyleSheet for Theme {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style, is_checked: bool) -> checkbox::Appearance {
        checkbox::Appearance {
            background: if is_checked {
                self.accent_color
                    .primary_fill_color(PrimaryFillColorVariant::Regular)
                    .to_background()
            } else {
                Background::Color(CHECKBOX_DARKER_GRAY)
            },
            border: Border {
                color: if is_checked {
                    self.accent_color
                        .border_color(BorderColorVariant::RegularColored)
                } else {
                    self.accent_color
                        .border_color(BorderColorVariant::RegularGrayscale)
                },
                radius: Radius::from(5.),
                width: 1.,
            },
            icon_color: Color::WHITE,
            text_color: Some(TEXT_COLOR_DEFAULT),
        }
    }

    fn hovered(&self, _style: &Self::Style, is_checked: bool) -> checkbox::Appearance {
        checkbox::Appearance {
            background: if is_checked {
                self.accent_color.secondary_fill_color().to_background()
            } else {
                Background::Color(CHECKBOX_LIGHTER_GRAY)
            },
            border: Border {
                color: if is_checked {
                    self.accent_color
                        .border_color(BorderColorVariant::HoveredColored)
                } else {
                    self.accent_color
                        .border_color(BorderColorVariant::HoveredGrayscale)
                },
                radius: Radius::from(5.),
                width: 1.,
            },
            icon_color: Color::WHITE,
            text_color: Some(TEXT_COLOR_HOVER),
        }
    }
}

impl Theme {
    pub fn checkbox<'a, Message>(
        &self,
        label: impl Into<String>,
        is_checked: bool,
    ) -> iced::widget::Checkbox<'a, Message> {
        checkbox(label, is_checked)
            .spacing(8)
            .style(iced::theme::Checkbox::Custom(Box::new(*self)))
    }
}
