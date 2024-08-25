//! Styles for `iced::widget::text::Text`

use super::prelude::*;

pub enum TextVariant {
    Regular,
    Dimmed,
}

pub trait TextExt<'a> {
    fn variant(self, variant: TextVariant) -> widget::text::Text<'a>;
}

impl<'a> TextExt<'a> for widget::Text<'a> {
    fn variant(self, variant: TextVariant) -> widget::text::Text<'a> {
        self.style(match variant {
            TextVariant::Regular => Color::BLACK,
            TextVariant::Dimmed => THEME.global.text.disabled.to_color(),
        })
    }
}
