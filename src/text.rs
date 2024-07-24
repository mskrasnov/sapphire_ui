use crate::theme::Theme;

use iced::widget::text;
use iced::widget::Text;
use iced::Color;
use iced::Pixels;

static TEXT_DIMMED: Color = Color::from_rgb(0.54, 0.55, 0.54);

pub enum TextVariant {
    Regular,
    Dimmed,
}

pub trait TextExt<'a> {
    fn variant(self, variant: TextVariant) -> Text<'a>;
}

impl<'a> TextExt<'a> for Text<'a> {
    fn variant(self, variant: TextVariant) -> Text<'a> {
        self.style(match variant {
            TextVariant::Dimmed => TEXT_DIMMED,
            TextVariant::Regular => Color::WHITE,
        })
    }
}

impl Theme {
    pub fn text<'a, L: ToString>(&self, label: L) -> Text<'a> {
        text(label).variant(TextVariant::Regular).size(Pixels(15.))
    }

    pub fn small_text<'a, L: ToString>(&self, label: L) -> Text<'a> {
        text(label).variant(TextVariant::Dimmed).size(Pixels(10.))
    }
}
