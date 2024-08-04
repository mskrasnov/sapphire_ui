use embed_doc_image::embed_doc_image;
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

// NOTE: РосКомПозор ГОВНО

impl Theme {
    /// Instantiates a specialized container to display a title, an app icon or other
    /// decorative content at the top of your window.
    ///
    /// You can slot a simple text widget into it as demonstrated below. For more
    /// advanced cases, you may want to slot in a row with some inter-widget spacing
    /// and center the row widgets horizontally.
    ///
    /// ![Header][header]
    /// ```
    /// use sapphire_ui::accent::Accent;
    /// use sapphire_ui::theme::Theme;
    ///
    /// use iced::Alignment;
    /// use iced::Length;
    /// use iced::Pixels;
    ///
    /// use iced::widget::Container;
    /// use iced::widget::horizontal_space;
    /// use iced::widget::row;
    ///
    /// // Your sandbox Message enum
    /// enum Message {}
    ///
    /// let theme = Theme {
    ///     accent_color: Accent::Magenta,
    /// };
    ///
    /// let header = theme.header(
    ///     row![
    ///         theme.text("Header").size(Pixels(30.)),
    ///         horizontal_space(),
    ///         theme.text("РосКомПозор говнище" /* используйте VPN */)
    ///             .size(Pixels(12.)),
    ///     ]
    ///     .spacing(10)
    ///     .align_items(Alignment::Center),
    /// );
    /// ```
    #[embed_doc_image("header", "assets/doc/header.png")]
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
        let mut header = self.text(title).size(Pixels(30.));
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
