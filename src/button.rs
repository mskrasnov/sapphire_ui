use embed_doc_image::embed_doc_image;
use iced::border::Border;
use iced::border::Radius;
use iced::widget::button;
use iced::Color;
use iced::Element;

use crate::accent::BorderColorVariant;
use crate::accent::ColorExt;
use crate::accent::PrimaryFillColorVariant;

use crate::common::*;
use crate::theme::Theme;

impl Theme {
    fn _default_button_appearance(&self) -> button::Appearance {
        button::Appearance {
            background: Some(
                self.accent_color
                    .primary_fill_color(PrimaryFillColorVariant::Regular)
                    .to_background(),
            ),
            border: Border {
                color: self
                    .accent_color
                    .border_color(BorderColorVariant::RegularColored),
                width: 1.,
                radius: Radius::from(5.),
            },
            text_color: TEXT_COLOR_DEFAULT,
            ..Default::default()
        }
    }
}

impl button::StyleSheet for Theme {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            ..self._default_button_appearance()
        }
    }

    fn disabled(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(FILL_DISABLED),
            border: Border {
                color: Color::from_rgba(0.2, 0.2, 0.2, 0.5),
                width: 1.,
                radius: Radius::from(5.),
            },
            text_color: TEXT_COLOR_DISABLED,
            ..self._default_button_appearance()
        }
    }

    fn hovered(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(
                self.accent_color
                    .primary_fill_color(PrimaryFillColorVariant::Hovered)
                    .to_background(),
            ),
            border: Border {
                color: self
                    .accent_color
                    .border_color(BorderColorVariant::HoveredColored),
                width: 1.,
                radius: Radius::from(5.),
            },
            text_color: TEXT_COLOR_HOVER,
            ..self._default_button_appearance()
        }
    }

    fn pressed(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(
                self.accent_color
                    .primary_fill_color(PrimaryFillColorVariant::Pressed)
                    .to_background(),
            ),
            border: Border {
                color: self
                    .accent_color
                    .border_color(BorderColorVariant::RegularColored),
                width: 1.,
                radius: Radius::from(5.),
            },
            text_color: TEXT_COLOR_PRESSED,
            ..self._default_button_appearance()
        }
    }
}

impl Theme {
    /// Instantiates a button widget.
    ///
    /// The button is disabled by default. You can enable it by chaining an
    /// `.on_press` call with a button press event to the button function.
    ///
    /// ![Button][button]
    ///
    /// ## Example
    /// ```
    /// use sapphire_ui::accent::Accent;
    /// use sapphire_ui::theme::Theme;
    ///
    /// // Your message event
    /// #[derive(Debug, Clone)]
    /// enum Message {
    ///     ButtonPressed,
    /// }
    ///
    /// let theme = Theme {
    ///     accent: Accent::Green,
    /// };
    /// let button = theme.button(theme.text("Click me!"))
    ///     .on_press(Message::ButtonPressed);
    /// ```
    #[embed_doc_image("button", "assets/doc/button.png")]
    pub fn button<'a, B>(&self, content: impl Into<Element<'a, B>>) -> iced::widget::Button<'a, B> {
        button(content)
            .padding([10., 20.])
            .style(iced::theme::Button::Custom(Box::new(*self)))
    }

    /// Simplified variant of [`crate::theme::Theme::button`] widget.
    pub fn button_text<'a, B, T: ToString>(&self, text: T) -> iced::widget::Button<'a, B> {
        button(self.text(text))
            .padding([10., 20.])
            .style(iced::theme::Button::Custom(Box::new(*self)))
    }
}
