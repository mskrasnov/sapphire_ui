// use iced::widget::button;
use iced::widget::container;
use iced::Element;
// use iced::Font;
use iced::Sandbox;
use iced::Settings;
use iced::Size;

use iced::widget::column;
use iced::widget::row;

use sapphire_ui::accent::Accent;
use sapphire_ui::theme::*;

fn main() -> iced::Result {
    let settings = Settings {
        window: iced::window::Settings {
            size: Size {
                width: 640.,
                height: 500.,
            },
            ..Default::default()
        },
        ..Default::default()
    };
    WidgetGallery::run(settings)
}

#[derive(Debug, Clone)]
enum Message {
    TitleChanged(String),
    SubtitleChanged(String),
    TextInputChanged(String),
    ButtonPressed,
    CheckStatusChanged(bool),
    ColorSchemeChanged(Accent),
}

struct WidgetGallery {
    theme: Theme,

    title: String,
    subtitle: String,
    text_input: String,
    button: String,
    check_status: bool,
}

impl Sandbox for WidgetGallery {
    type Message = Message;

    fn new() -> Self {
        Self {
            theme: Theme {
                accent_color: Accent::Magenta,
            },
            title: "Widget Gallery".to_string(),
            subtitle: "".to_string(),
            text_input: "".to_string(),
            button: "Press me!".to_string(),
            check_status: true,
        }
    }

    fn title(&self) -> String {
        format!("Sapphire UI Kit Widget Gallery")
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let subtitle = if self.subtitle.is_empty() {
            None
        } else {
            Some(&self.subtitle)
        };

        let header = self.theme.header_title(&self.title, subtitle);

        let table_headers = column![
            self.theme
                .text("Enabled inputs")
                .variant(TextVariant::Dimmed),
            self.theme
                .text("Sapphire UI version")
                .variant(TextVariant::Dimmed),
            self.theme.text("Accent color").variant(TextVariant::Dimmed),
            self.theme.text("Text input").variant(TextVariant::Dimmed),
        ]
        .align_items(iced::Alignment::End)
        .spacing(2);

        let table_values = column![
            self.theme.text(match self.check_status {
                true => "yes",
                false => "no",
            }),
            self.theme.text(env!("CARGO_PKG_VERSION")),
            self.theme.text(self.theme.accent_color),
            self.theme.text(&self.text_input),
        ]
        .spacing(2);

        let mut button = self.theme.button_text(&self.button);
        let mut text_input = self
            .theme
            .text_input("Put some text here!", &self.text_input);

        if self.check_status {
            button = button.on_press(Message::ButtonPressed);
            text_input = text_input.on_input(Message::TextInputChanged);
        }

        let accent = Some(self.theme.accent_color);
        let accent_color = column![
            self.theme.small_text("Select accent color:"),
            self.theme.radio(
                Accent::Magenta.to_string(),
                Accent::Magenta,
                accent,
                Message::ColorSchemeChanged,
            ),
            self.theme.radio(
                Accent::Cyan.to_string(),
                Accent::Cyan,
                accent,
                Message::ColorSchemeChanged,
            ),
            self.theme.radio(
                Accent::Green.to_string(),
                Accent::Green,
                accent,
                Message::ColorSchemeChanged,
            ),
        ]
        .spacing(5);

        let body = container(
            column![
                self.theme
                    .checkbox("Enable inputs", true)
                    .on_toggle(Message::CheckStatusChanged),
                row![text_input, button,].spacing(5),
                column![
                    self.theme.small_text("Some info:"),
                    row![table_headers, table_values,].spacing(10),
                ],
                row![
                    self.theme
                        .text_input("Enter header name...", &self.title)
                        .on_input(Message::TitleChanged),
                    self.theme
                        .text_input("Enter subtitle name here", &self.subtitle)
                        .on_input(Message::SubtitleChanged),
                ]
                .spacing(5),
                accent_color,
            ]
            .spacing(20),
        )
        .padding(20);

        let contents = column![header, body].spacing(5);

        self.theme.primary_container(contents).into()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::TitleChanged(title) => {
                self.title = title;
            }
            Message::SubtitleChanged(subtitle) => {
                self.subtitle = subtitle;
            }
            Message::TextInputChanged(text) => {
                self.text_input = text;
            }
            Message::ButtonPressed => {
                self.button = "Button pressed!".to_string();
            }
            Message::CheckStatusChanged(check) => {
                self.check_status = check;
                self.text_input = "Oh no!".to_string();
            }
            Message::ColorSchemeChanged(scheme) => {
                self.theme.accent_color = scheme;
            }
        }
    }
}
