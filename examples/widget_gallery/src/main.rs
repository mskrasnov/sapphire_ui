use iced::Alignment;
use iced::Element;
use iced::Length;
use iced::Sandbox;
use iced::Settings;
use iced::Size;

use iced::widget::column;
use iced::widget::horizontal_space;
use iced::widget::row;

use sapphire_ui::theme::text::TextExt;
use sapphire_ui::theme::text::TextVariant;
use sapphire_ui::widgets::button;
use sapphire_ui::widgets::checkbox;
use sapphire_ui::widgets::container;
use sapphire_ui::widgets::progress_bar;
use sapphire_ui::widgets::radio;
use sapphire_ui::widgets::scrollable;
use sapphire_ui::widgets::slider;
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
    level: u8,
    enable_inputs: bool,
    use_dark: bool,
}

#[derive(Debug, Clone)]
enum Message {
    TitleChanged(String),
    SubtitleChanged(String),
    SomeTextChanged(String),
    InputsChanged(bool),
    RadioChanged(Radio),
    LevelChanged(u8),
    UseDarkToggled(bool),
    LevelDown,
    LevelUp,
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
            level: 10,
            enable_inputs: true,
            use_dark: false,
        }
    }

    fn title(&self) -> String {
        format!("Widget Gallery")
    }

    fn theme(&self) -> iced::Theme {
        if self.use_dark {
            iced::Theme::Dark
        } else {
            iced::Theme::Light
        }
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
            }
            Message::RadioChanged(rd) => {
                self.radio = rd;
                match rd {
                    Radio::True => self.enable_inputs = true,
                    Radio::False => self.enable_inputs = false,
                }
            }
            Message::LevelChanged(level) => self.level = level,
            Message::LevelDown => self.level -= 1,
            Message::LevelUp => self.level += 1,
            Message::UseDarkToggled(dark) => self.use_dark = dark,
            Message::ButtonPressed => self.some_text = String::new(),
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let title = widget_group(
            row![
                column![
                    title_text(&self.title),
                    text(&self.subtitle).variant(TextVariant::Dimmed)
                ]
                .spacing(5)
                .width(Length::Fill),
                checkbox("Dark style", self.use_dark).on_toggle(Message::UseDarkToggled)
            ]
            .align_items(Alignment::Center),
        );

        let check_inputs = row![
            checkbox("Enable more inputs", self.enable_inputs).on_toggle(Message::InputsChanged),
            horizontal_space(),
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
        .spacing(10)
        .align_items(Alignment::Center);

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
        // .spacing(5)
        .align_items(Alignment::End);

        let values_data = column![
            text(if self.enable_inputs { "yes" } else { "no" }),
            text(env!("CARGO_PKG_VERSION")),
            text(&self.some_text),
        ]
        /*.spacing(5)*/;

        let level_down = if self.level == 0 {
            button("Down")
        } else {
            button("Down").on_press(Message::LevelDown)
        };

        let level_up = if self.level == 200 {
            button("Up")
        } else {
            button("Up").on_press(Message::LevelUp)
        };

        let level = column![
            slider(0..=200, self.level, Message::LevelChanged),
            row![level_up, text(self.level), level_down,]
                .spacing(5)
                .align_items(Alignment::Center),
            progress_bar(0.0..=200., self.level as f32),
        ]
        .spacing(5)
        .align_items(Alignment::Center)
        .width(150);

        let values_table = column![
            small_text("Some info:"),
            row![
                widget_group(row![values_titles, values_data].spacing(5)).width(Length::Fill),
                level,
            ]
            .spacing(5),
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
            check_inputs,
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
