use crate::theme::prelude::*;
use crate::theme::slider::SliderStyle;

use std::ops::RangeInclusive;
use embed_doc_image::embed_doc_image;

/// Creates `iced::widget::slider::Slider` widget
///
/// ![slider][slider]
#[embed_doc_image("slider", "assets/doc/slider.png")]
pub fn slider<'a, T, Message>(
    range: RangeInclusive<T>,
    value: T,
    on_change: impl Fn(T) -> Message + 'a,
) -> widget::Slider<'a, T, Message>
where
    T: Copy + From<u8> + PartialOrd,
    Message: Clone,
{
    widget::slider(range, value, on_change).style(theme::Slider::Custom(Box::new(SliderStyle)))
}

pub fn vertical_slider<'a, T, Message>(
    range: RangeInclusive<T>,
    value: T,
    on_change: impl Fn(T) -> Message + 'a,
) -> widget::VerticalSlider<'a, T, Message>
where
    T: Copy + From<u8> + PartialOrd,
    Message: Clone,
{
    widget::vertical_slider(range, value, on_change).style(theme::Slider::Custom(Box::new(SliderStyle)))
}
