//! Declaration of the theme's config structure

use lazy_static::lazy_static;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

use iced::Background;
use iced::Color;

pub mod prelude;

pub mod button;
pub mod container;
pub mod scrollable;
pub mod text;
pub mod text_input;

lazy_static! {
    pub static ref THEME: Theme = Theme::default();
}

pub type WidgetName = String;
pub type ColorName = String;

#[derive(Debug, Deserialize, Serialize)]
pub struct Theme {
    pub meta: Meta,
    pub global: GlobalStyle,
    pub widget: HashMap<WidgetName, WidgetStyle>,
}

impl Default for Theme {
    fn default() -> Self {
        let content = include_str!("../themes/default.toml");
        toml::from_str(&content).unwrap()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Meta {
    pub id: String,
    pub variant: ThemeVariant,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub enum ThemeVariant {
    #[serde(rename = "light")]
    Light,

    #[default]
    #[serde(rename = "dark")]
    Dark,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GlobalStyle {
    pub container_background: RGB,
    pub primary_fill_color: PrimaryFillColor,
    pub secondary_fill_color: RGB,
    pub border: Border,
    pub text: Text,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct RGB(u8, u8, u8);

#[derive(Debug, Deserialize, Serialize)]
pub struct PrimaryFillColor {
    pub regular: RGB,
    pub hovered: RGB,
    pub pressed: RGB,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Border {
    pub regular_grayscale: RGB,
    pub regular_colored: RGB,
    pub hovered_grayscale: RGB,
    pub hovered_colored: RGB,
    pub focused: RGB,
    pub width: f32,
    pub radius: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Text {
    pub default: RGB,
    pub hover: RGB,
    pub pressed: RGB,
    pub disabled: RGB,
    pub size: f32,
    pub font: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WidgetStyle {
    pub background: Option<RGB>,
    pub border: Option<Border>,
    pub icon_color: Option<RGB>,
    pub text: Option<Text>,
    pub padding: Option<(u16, u16)>,
    pub lighter_color: Option<RGB>,
    pub darker_color: Option<RGB>,
    pub colors: Option<HashMap<ColorName, RGB>>,
}

impl RGB {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self(r, g, b /*None*/)
    }

    pub fn to_color(&self) -> Color {
        Color::from_rgb8(self.0, self.1, self.2)
    }

    pub fn to_background(&self) -> Background {
        Background::Color(self.to_color())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        dbg!(Theme::default());
    }
}
