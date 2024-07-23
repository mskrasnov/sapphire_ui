use iced::Font;
use iced::Size;

use sapphire_ui::accent::Accent;
use sapphire_ui::theme::*;

use iced::widget::column;
use iced::widget::container;

use iced::Element;
use iced::Sandbox;
use iced::Settings;

fn main() -> iced::Result {
    let settings = Settings {
        window: iced::window::Settings {
            size: Size::new(400., 520.),
            ..Default::default()
        },
        antialiasing: true,
        default_font: Font::with_name("Cantarell"),
        ..Settings::default()
    };
    Text::run(settings)
}

#[derive(Debug, Clone)]
enum Message {
    TextChanged(String),
    SubtextChanged(String),
    ButtonPressed,
}

struct Text {
    theme: Theme,
    text: String,
    subtext: String,
    button_text: String,
}

impl Sandbox for Text {
    type Message = Message;

    fn new() -> Self {
        Self {
            theme: Theme {
                accent_color: Accent::Magenta,
            },
            text: format!("I love Rust and Iced! (title)"),
            subtext: format!("ЖОПА В КОСМОСЕ (subtitle)"),
            button_text: format!("Press me!"),
        }
    }

    fn title(&self) -> String {
        format!("Sapphire UI Kit Demo")
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let subtext = if &self.subtext == "" {
            None
        } else {
            Some(&self.subtext)
        };

        let header = self.theme.header_title(&self.text, subtext);
        let input = self
            .theme
            .text_input("Put some text here...", &self.text)
            .on_input(Message::TextChanged);
        let subinput = self
            .theme
            .text_input("Put some text here...", &self.subtext)
            .on_input(Message::SubtextChanged);

        let button = self
            .theme
            .button_text(&self.button_text)
            .on_press(Message::ButtonPressed);

        let body = column![
            header,
            container(column![input, subinput, button,].spacing(10)).padding(20),
        ]
        .spacing(5);

        self.theme.primary_container(body).into()
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Message::TextChanged(text) => {
                self.text = text;
            }
            Message::SubtextChanged(subtext) => {
                self.subtext = subtext;
            }
            Message::ButtonPressed => {
                if &self.button_text != "НАЖАТО" {
                    self.button_text = format!("НАЖАТО");
                    self.theme = Theme {
                        accent_color: Accent::Green,
                    };
                } else {
                    self.button_text = format!("Ещё раз нажато");
                    self.theme = Theme {
                        accent_color: Accent::Cyan,
                    };
                }

                if &self.button_text == "Ещё раз нажато" {
                    self.button_text = format!("ХВАТИТ");
                    self.theme = Theme {
                        accent_color: Accent::Magenta,
                    };
                }
            }
        }
    }
}
