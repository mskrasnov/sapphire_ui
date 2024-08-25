use iced::widget::column;
use iced::Element;
use iced::Sandbox;
use iced::Settings;
use iced::Size;

use sapphire_ui::widgets::button;
use sapphire_ui::widgets::container;
use sapphire_ui::widgets::text;

fn main() -> iced::Result {
    ButtonApplication::run(Settings {
        window: iced::window::Settings {
            size: Size {
                width: 200.,
                height: 200.,
            },
            ..Default::default()
        },
        ..Default::default()
    })
}

#[derive(Debug, Clone, Copy)]
enum Message {
    ButtonPressed,
}

struct ButtonApplication {
    show_text: bool,
}

impl Sandbox for ButtonApplication {
    type Message = Message;

    fn new() -> Self {
        Self { show_text: false }
    }

    fn title(&self) -> String {
        format!("Button example")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::ButtonPressed => self.show_text = !self.show_text,
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let button = button(text("Show text message")).on_press(Message::ButtonPressed);
        let text = text("Some text...\nEnjoy!");

        let mut ui = column![button,].spacing(5);

        if self.show_text {
            ui = ui.push(text);
        }

        container(ui).center_x().center_y().into()
    }
}
