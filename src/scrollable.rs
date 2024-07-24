// use iced::application::Appearance;
use iced::border::Radius;
use iced::widget::scrollable;
use iced::widget::scrollable::StyleSheet;
use iced::Border;
// use iced::Color;
use iced::Element;

use crate::accent::BorderColorVariant;
// use crate::accent::ColorExt;
use crate::accent::PrimaryFillColorVariant;

use crate::theme::Theme;

impl Theme {
    fn _default_scrollbar_appearance(&self) -> scrollable::Appearance {
        scrollable::Appearance {
            container: iced::widget::container::Appearance {
                background: Some(self.accent_color.primary_container_background()),
                ..Default::default()
            },
            gap: None,
            scrollbar: scrollable::Scrollbar {
                background: None,
                border: Border {
                    color: self
                        .accent_color
                        .border_color(BorderColorVariant::RegularColored),
                    width: 1.,
                    radius: Radius::from(5.),
                },
                scroller: scrollable::Scroller {
                    color: self
                        .accent_color
                        .primary_fill_color(PrimaryFillColorVariant::Regular),
                    border: Border {
                        color: self.accent_color.secondary_fill_color(),
                        width: 1.,
                        radius: Radius::from(5.),
                    },
                },
            },
        }
    }
}

impl scrollable::StyleSheet for Theme {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> scrollable::Appearance {
        scrollable::Appearance {
            ..self._default_scrollbar_appearance()
        }
    }

    fn hovered(
        &self,
        _style: &Self::Style,
        is_mouse_over_scrollbar: bool,
    ) -> scrollable::Appearance {
        scrollable::Appearance {
            container: iced::widget::container::Appearance {
                background: Some(self.accent_color.primary_container_background()),
                ..Default::default()
            },
            gap: None,
            scrollbar: scrollable::Scrollbar {
                background: None,
                border: Border {
                    color: self
                        .accent_color
                        .border_color(match is_mouse_over_scrollbar {
                            true => BorderColorVariant::HoveredColored,
                            false => BorderColorVariant::HoveredGrayscale,
                        }),
                    width: 1.,
                    radius: Radius::from(5.),
                },
                scroller: scrollable::Scroller {
                    color: self
                        .accent_color
                        .primary_fill_color(PrimaryFillColorVariant::Hovered),
                    border: Border {
                        color: self.accent_color.secondary_fill_color(),
                        width: 1.,
                        radius: Radius::from(5.),
                    },
                },
            },
        }
    }
}

impl Theme {
    pub fn scrollable<'a, Message, Theme>(
        &self,
        content: impl Into<Element<'a, Message, Theme>>,
    ) -> iced::widget::Scrollable<'a, Message, Theme>
    where
        Theme: StyleSheet,
        <Theme as iced::widget::scrollable::StyleSheet>::Style: From<iced::theme::Scrollable>,
    {
        scrollable(content).style(iced::theme::Scrollable::Custom(Box::new(*self)))
    }
}
