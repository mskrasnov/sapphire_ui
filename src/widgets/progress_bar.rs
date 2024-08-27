use std::ops::RangeInclusive;

use crate::theme::prelude::*;
use crate::theme::progress_bar::ProgressBarStyle;

use embed_doc_image::embed_doc_image;

/// Creates `iced::widget::progress_bar::ProgressBar` widget
///
/// ![progress_bar][progress_bar]
#[embed_doc_image("progress_bar", "assets/doc/progress_bar.png")]
pub fn progress_bar(range: RangeInclusive<f32>, value: f32) -> widget::ProgressBar {
    widget::progress_bar(range, value)
        .style(theme::ProgressBar::Custom(Box::new(ProgressBarStyle)))
        .height(10)
}
