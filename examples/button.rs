use sapphire_ui::theme::prelude::*;
use sapphire_ui::widgets::button::button;
use sapphire_ui::widgets::container::container;

use iced::widget::text;
use iced::{Font, Sandbox, Settings};

fn main() -> iced::Result {
    let settings = Settings {
        default_text_size: Pixels(THEME.global.text.size),
        default_font: Font::with_name(&THEME.global.text.font),
        ..Default::default()
    };
    ButtonExample::run(settings)
}

#[derive(Clone, Copy, Debug)]
enum Message {
    ButtonPressed,
}

struct ButtonExample {
    label: String,
    pressed: bool,
}

impl Sandbox for ButtonExample {
    type Message = Message;

    fn new() -> Self {
        Self {
            label: "Press me!".to_string(),
            pressed: false,
        }
    }

    fn title(&self) -> String {
        format!("Button example")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::ButtonPressed => {
                self.label = "Pressed!".to_string();
                self.pressed = false;
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let btn = if !self.pressed {
            button(text(&self.label)).on_press(Message::ButtonPressed)
        } else {
            button(text(&self.label))
        };

        container(btn).into()
    }
}
