use iced::widget::container;
use iced::widget::Container;

use iced::Element;
use iced::Length;

use crate::theme::Theme;

impl Theme {
    pub fn primary_container<'a, T>(self, contents: impl Into<Element<'a, T>>) -> Container<'a, T> {
        container(contents)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(move |_: &_| container::Appearance {
                background: Some(self.accent_color.primary_container_background()),
                ..Default::default()
            })
    }
}