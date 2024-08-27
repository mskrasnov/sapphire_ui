use std::ops::RangeInclusive;

use crate::theme::prelude::*;
use crate::theme::progress_bar::ProgressBarStyle;

pub fn progress_bar(range: RangeInclusive<f32>, value: f32) -> widget::ProgressBar {
    widget::progress_bar(range, value)
        .style(theme::ProgressBar::Custom(Box::new(ProgressBarStyle)))
        .height(10)
}
