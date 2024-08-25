use iced::widget::column;
use iced::widget::row;
use iced::Element;
use iced::Sandbox;
use iced::Settings;
use iced::Size;

use sapphire_ui::theme::text::TextExt;
use sapphire_ui::theme::text::TextVariant;
use sapphire_ui::widgets::container;
use sapphire_ui::widgets::text;
use sapphire_ui::widgets::text_input;

fn main() -> iced::Result {
    TextInputApplication::run(Settings {
        window: iced::window::Settings {
            size: Size {
                width: 300.,
                height: 300.,
            },
            ..Default::default()
        },
        ..Default::default()
    })
}

#[derive(Debug, Clone)]
enum Message {
    FirstTextChanged(String),
    SecondTextChanged(String),
}

struct TextInputApplication {
    first_text: String,
    second_text: String,
}

impl Sandbox for TextInputApplication {
    type Message = Message;

    fn new() -> Self {
        Self {
            first_text: String::new(),
            second_text: String::new(),
        }
    }

    fn title(&self) -> String {
        format!("Text input example")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::FirstTextChanged(text) => self.first_text = text,
            Message::SecondTextChanged(text) => self.second_text = text,
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let first_text_input = text_input("Enter some text here", &self.first_text)
            .on_input(Message::FirstTextChanged);

        let second_text_input = text_input("Enter some text here", &self.second_text)
            .on_input(Message::SecondTextChanged);

        let value_table = column![
            row![
                text("First text:").variant(TextVariant::Dimmed),
                text(&self.first_text),
            ]
            .spacing(5),
            row![
                text("Second text:").variant(TextVariant::Dimmed),
                text(&self.second_text),
            ]
            .spacing(5),
        ]
        .align_items(iced::Alignment::Center)
        .spacing(5);

        let ui = column![first_text_input, second_text_input, value_table,]
            .spacing(10)
            .max_width(250.);

        container(ui).center_x().center_y().into()
    }
}
