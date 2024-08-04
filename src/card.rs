use std::fmt::Display;

use iced::Color;

use iced_aw::widgets::card;
use iced_aw::Card;

use crate::accent::BorderColorVariant;
use crate::accent::ColorExt;
use crate::accent::PrimaryFillColorVariant;

use crate::common::*;
use crate::theme::Theme;

impl Theme {
    fn _default_card_appearance(&self) -> card::Appearance {
        card::Appearance {
            background: self
                .accent_color
                .primary_fill_color(PrimaryFillColorVariant::Regular)
                .to_background(),
            border_radius: 5.,
            border_width: 1.,
            border_color: self
                .accent_color
                .border_color(BorderColorVariant::RegularColored),
            head_background: self.accent_color.primary_container_background(),
            head_text_color: Color::from_rgb(0., 0., 0.),
            body_background: self.accent_color.primary_container_background(),
            body_text_color: TEXT_COLOR_DEFAULT,
            foot_background: self.accent_color.primary_container_background(),
            ..Default::default()
        }
    }
}

impl card::StyleSheet for Theme {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> card::Appearance {
        card::Appearance {
            ..self._default_card_appearance()
        }
    }
}

impl Theme {
    pub fn card<'a, Message, Theme, H, C>(&self, header: H, content: C) -> Card<'a, Theme>
    where
        // Theme: card::StyleSheet,
        H: Display,
        C: Display,
    {
        card(self.text(&header), self.text(&content))
            .style(iced_aw::style::card::CardStyles::Custom(Box::new(*self)))
    }
}
