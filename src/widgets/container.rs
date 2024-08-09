use crate::theme::container::ContainerStyle;
use crate::theme::prelude::*;

pub fn container<'a, T>(contents: impl Into<Element<'a, T>>) -> widget::Container<'a, T> {
    widget::container(contents)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(theme::Container::Custom(Box::new(ContainerStyle)))
}
