use iced::Alignment;
use iced::Element;
use iced::Length;
use iced::Sandbox;
use iced::Settings;
use iced::Size;

use iced::widget::column;
use iced::widget::row;
use iced::widget::horizontal_space;

use sapphire_ui::theme::text::TextExt;
use sapphire_ui::theme::text::TextVariant;
use sapphire_ui::widgets::button;
use sapphire_ui::widgets::checkbox;
use sapphire_ui::widgets::container;
use sapphire_ui::widgets::radio;
use sapphire_ui::widgets::scrollable;
use sapphire_ui::widgets::small_text;
use sapphire_ui::widgets::text;
use sapphire_ui::widgets::text_input;
use sapphire_ui::widgets::title_text;
use sapphire_ui::widgets::widget_group;

fn main() -> iced::Result {
    WGApplication::run(Settings {
        window: iced::window::Settings {
            size: Size {
                width: 640.,
                height: 500.,
            },
            ..Default::default()
        },
        ..Default::default()
    })
}

struct WGApplication {
    title: String,
    subtitle: String,
    some_text: String,
    radio: Radio,
    enable_inputs: bool,
}

#[derive(Debug, Clone)]
enum Message {
    TitleChanged(String),
    SubtitleChanged(String),
    SomeTextChanged(String),
    InputsChanged(bool),
    RadioChanged(Radio),
    ButtonPressed,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Radio {
    True,
    False,
}

impl Sandbox for WGApplication {
    type Message = Message;

    fn new() -> Self {
        Self {
            title: format!("Title"),
            subtitle: format!("Subtitle"),
            some_text: String::new(),
            radio: Radio::True,
            enable_inputs: true,
        }
    }

    fn title(&self) -> String {
        format!("Widget Gallery")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::TitleChanged(title) => self.title = title,
            Message::SubtitleChanged(subtitle) => self.subtitle = subtitle,
            Message::SomeTextChanged(text) => self.some_text = text,
            Message::InputsChanged(status) => {
                self.enable_inputs = status;
                if status {
                    self.radio = Radio::True;
                } else {
                    self.radio = Radio::False;
                }
            },
            Message::RadioChanged(rd) => {
                self.radio = rd;
                match rd {
                    Radio::True => self.enable_inputs = true,
                    Radio::False => self.enable_inputs = false,
                }
            }
            Message::ButtonPressed => self.some_text = String::new(),
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let title = widget_group(
            column![
                title_text(&self.title),
                text(&self.subtitle).variant(TextVariant::Dimmed)
            ]
            .spacing(5)
            .width(Length::Fill),
        );

        let check_inputs =
            checkbox("Enable more inputs", self.enable_inputs).on_toggle(Message::InputsChanged);

        let inputs_radio = row![
            text("Inputs status:"),
            radio(
                "Enabled",
                Radio::True,
                Some(self.radio),
                Message::RadioChanged
            ),
            radio(
                "Disabled",
                Radio::False,
                Some(self.radio),
                Message::RadioChanged
            ),
        ]
        .spacing(10);

        let title_inputs = row![
            text_input("Enter title name here...", &self.title).on_input(Message::TitleChanged),
            text_input("Enter subtitle name here...", &self.subtitle)
                .on_input(Message::SubtitleChanged),
        ]
        .spacing(5);

        let some_text_input = if self.enable_inputs {
            text_input("Enter some text here...", &self.some_text)
                .on_input(Message::SomeTextChanged)
        } else {
            text_input("Enter some text here...", &self.some_text)
        };

        let some_text_clean = if self.enable_inputs && !self.some_text.is_empty() {
            button("Clean").on_press(Message::ButtonPressed)
        } else {
            button("Clean")
        };

        let values_titles = column![
            text("More inputs:").variant(TextVariant::Dimmed),
            text("Sapphire UI ver.:").variant(TextVariant::Dimmed),
            text("Some text:").variant(TextVariant::Dimmed),
        ]
        .spacing(5)
        .align_items(Alignment::End);

        let values_data = column![
            text(if self.enable_inputs { "yes" } else { "no" }),
            text(env!("CARGO_PKG_VERSION")),
            text(&self.some_text),
        ]
        .spacing(5);

        let values_table = column![
            small_text("Some info:"),
            widget_group(row![values_titles, values_data].spacing(5)).width(Length::Fill),
        ]
        .spacing(5);

        let text_scrollable = scrollable(text(include_str!("/proc/meminfo")).width(Length::Fill));

        let big_text = column![
            small_text("Big text:"),
            widget_group(text_scrollable).width(Length::Fill),
        ]
        .spacing(5);

        let ui = column![
            title,
            row![check_inputs, horizontal_space(), inputs_radio]
                .spacing(5)
                .align_items(Alignment::Center),
            title_inputs,
            row![some_text_input, some_text_clean,]
                .spacing(5)
                .align_items(Alignment::Center),
            values_table,
            big_text,
        ]
        .spacing(10);
        container(ui).padding([10, 10]).center_x().into()
    }
}
