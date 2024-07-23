use iced::widget::column;
use iced::widget::container;
use iced::widget::row;
use iced::widget::Container;

use iced::Element;
use iced::Length;
use iced::Pixels;

use crate::accent::ColorExt;
use crate::accent::PrimaryFillColorVariant;
use crate::text::TextExt;
use crate::theme::Theme;

impl Theme {
    pub fn header<'a, T: 'a>(self, contents: impl Into<Element<'a, T>>) -> Container<'a, T> {
        let header_main = container(contents)
            .width(Length::Fill)
            .style(move |_: &_| container::Appearance {
                background: Some(
                    self.accent_color
                        .primary_fill_color(PrimaryFillColorVariant::Regular)
                        .to_background(),
                ),
                ..Default::default()
            })
            .padding([10, 20]);

        let header_separator = container(row![])
            .width(Length::Fill)
            .height(Pixels(1.))
            .style(move |_: &_| container::Appearance {
                background: Some(self.accent_color.secondary_fill_color().to_background()),
                ..Default::default()
            });

        container(column![header_main, header_separator])
    }

    pub fn header_title<'a, M: 'a, T: ToString, S: ToString>(
        self,
        title: T,
        subtitle: Option<S>,
    ) -> Container<'a, M> {
        let mut header = self
            .text(title)
            .size(Pixels(30.));
        let mut content = column![header.clone(),];

        if let Some(subtitle) = subtitle {
            header = header.size(Pixels(20.));
            let subheader = self
                .text(subtitle)
                .size(Pixels(15.))
                .variant(crate::text::TextVariant::Dimmed);
            content = column![header, subheader]
        }

        self.header(content)
    }
}
