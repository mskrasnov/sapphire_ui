use serde::Deserialize;
use serde::Serialize;
use toml;
use std::fs;

use std::collections::HashMap;
use crate::accent::Accent;

// use lazy_static::lazy_static;

// lazy_static! {}

pub type RGB = (f32, f32, f32);

#[derive(Debug, Deserialize, Serialize)]
pub struct ThemeConfig {
    pub style: ThemeStyle,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ThemeStyle {
    Light,
    Dark,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ThemeData {
    pub primary_container_background: PrimaryContainerBackground,
    pub primary_fill_color: RGB,
    pub secondary_fill_color: SecondaryFillColor,
    pub border_color: BorderColor,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PrimaryContainerBackground {
    pub regular: RGB,
    pub hovered: RGB,
    pub pressed: RGB,
}

/**************************************************************/

impl Default for ThemeStyle {
    fn default() -> Self {
        Self::Dark
    }
}
