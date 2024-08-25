use crate::theme::container::ContainerStyle;
use crate::theme::container::WidgetGroup;
use crate::theme::prelude::*;

use embed_doc_image::embed_doc_image;

pub fn container<'a, T>(contents: impl Into<Element<'a, T>>) -> widget::Container<'a, T> {
    widget::container(contents)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(theme::Container::Custom(Box::new(ContainerStyle)))
}

/// `widget_group` is a widget for combining other widgets into a single
/// frame.
///
/// ![widget_group][widget_group]
#[embed_doc_image("widget_group", "assets/doc/widget_group.png")]
pub fn widget_group<'a, T>(widgets: impl Into<Element<'a, T>>) -> widget::Container<'a, T> {
    widget::container(widgets)
        .padding([10, 10])
        .style(theme::Container::Custom(Box::new(WidgetGroup)))
}
