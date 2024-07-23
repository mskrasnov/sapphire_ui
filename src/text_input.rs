use crate::accent::BorderColorVariant;
use crate::theme::Theme;
use crate::common::*;

use iced::border::Radius;
use iced::Border;
use iced::Color;

use iced::widget::text_input;
use iced::widget::TextInput;

impl Theme {
    fn _default_text_input_appearance(&self) -> text_input::Appearance {
        text_input::Appearance {
            background: iced::Background::Color(Color::from_rgb(0.05, 0.05, 0.05)),
            icon_color: Color::WHITE,
            border: Border {
                width: 1.,
                color: self
                    .accent_color
                    .border_color(BorderColorVariant::RegularGrayscale),
                radius: Radius::from(5.),
            },
        }
    }
}

impl text_input::StyleSheet for Theme {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            ..self._default_text_input_appearance()
        }
    }

    fn disabled(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: FILL_DISABLED,
            ..self._default_text_input_appearance()
        }
    }

    fn disabled_color(&self, _style: &Self::Style) -> Color {
        TEXT_COLOR_DISABLED
    }

    fn focused(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            ..self._default_text_input_appearance()
        }
    }

    fn hovered(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            ..self._default_text_input_appearance()
        }
    }

    fn placeholder_color(&self, _style: &Self::Style) -> Color {
        Color::from_rgb(0.5, 0.5, 0.5)
    }

    fn selection_color(&self, _style: &Self::Style) -> Color {
        Color::from_rgb(0.25, 0.25, 0.25)
    }

    fn value_color(&self, _style: &Self::Style) -> Color {
        TEXT_COLOR_DEFAULT
    }
}

impl Theme {
    pub fn text_input<'a, MessageType: Clone>(
        &self,
        placeholder: &str,
        label: &str,
    ) -> TextInput<'a, MessageType> {
        text_input(placeholder, label)
            .padding([10, 15])
            .style(iced::theme::TextInput::Custom(Box::new(*self)))
    }
}
