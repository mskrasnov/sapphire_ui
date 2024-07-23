use iced::widget::radio;

use iced::Background;
use iced::Color;
use iced::Pixels;

use crate::accent::BorderColorVariant;
use crate::accent::ColorExt;
use crate::accent::PrimaryFillColorVariant;

use crate::common::TEXT_COLOR_DEFAULT;
use crate::common::TEXT_COLOR_HOVER;

use crate::theme::Theme;

static RADIO_DARKER_GRAY: Color = Color::from_rgb(0.11, 0.11, 0.11);
static RADIO_LIGHTER_GRAY: Color = Color::from_rgb(0.12, 0.12, 0.12);

impl Theme {
    fn _default_radio_appearance(&self, is_checked: bool) -> radio::Appearance {
        radio::Appearance {
            background: match is_checked {
                true => self
                    .accent_color
                    .primary_fill_color(PrimaryFillColorVariant::Regular)
                    .to_background(),
                false => Background::Color(RADIO_DARKER_GRAY),
            },
            border_color: match is_checked {
                true => self
                    .accent_color
                    .border_color(BorderColorVariant::RegularColored),
                false => self
                    .accent_color
                    .border_color(BorderColorVariant::RegularGrayscale),
            },
            dot_color: Color::WHITE,
            border_width: 1.0,
            text_color: Some(TEXT_COLOR_DEFAULT),
        }
    }
}

impl radio::StyleSheet for Theme {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style, is_checked: bool) -> radio::Appearance {
        self._default_radio_appearance(is_checked)
    }

    fn hovered(&self, _style: &Self::Style, is_checked: bool) -> radio::Appearance {
        radio::Appearance {
            background: match is_checked {
                true => self.accent_color.secondary_fill_color().to_background(),
                false => Background::Color(RADIO_LIGHTER_GRAY),
            },
            border_color: match is_checked {
                true => self
                    .accent_color
                    .border_color(BorderColorVariant::HoveredColored),
                false => self
                    .accent_color
                    .border_color(BorderColorVariant::HoveredGrayscale),
            },
            dot_color: Color::WHITE,
            border_width: 1.0,
            text_color: Some(TEXT_COLOR_HOVER),
        }
    }
}

impl Theme {
    pub fn radio<T, M, V>(
        &self,
        label: impl Into<String>,
        value: V,
        selected: Option<V>,
        on_click: impl FnOnce(V) -> M,
    ) -> iced::widget::Radio<M, T>
    where
        T: radio::StyleSheet,
        M: Clone,
        V: Copy + Eq,
        <T as iced::widget::radio::StyleSheet>::Style: From<iced::theme::Radio>,
    {
        radio(label, value, selected, on_click)
            .size(Pixels(20.))
            .spacing(8)
            .style(iced::theme::Radio::Custom(Box::new(*self)))
    }
}
