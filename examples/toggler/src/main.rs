use iced::widget::column;
use iced::Element;
use iced::Sandbox;
use iced::Settings;
use iced::Size;

use sapphire_ui::widgets::checkbox;
use sapphire_ui::widgets::container;
use sapphire_ui::widgets::toggler;
use sapphire_ui::widgets::widget_group;

fn main() -> iced::Result {
    TogglerApplication::run(Settings {
        window: iced::window::Settings {
            size: Size {
                width: 300.,
                height: 200.,
            },
            ..Default::default()
        },
        ..Default::default()
    })
}

#[derive(Debug, Clone, Copy)]
enum Message {
    TogglerChanged(bool),
}

struct TogglerApplication {
    toggle: bool,
}

impl Sandbox for TogglerApplication {
    type Message = Message;

    fn new() -> Self {
        Self { toggle: true }
    }

    fn title(&self) -> String {
        format!("Toggler example")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::TogglerChanged(status) => self.toggle = status,
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let toggle = toggler(
            "Toggle value".to_string(),
            self.toggle,
            Message::TogglerChanged,
        );
        let check = checkbox("Toggle value", self.toggle).on_toggle(Message::TogglerChanged);

        let toggle_check = column![toggle, check,]
            // .align_items(iced::Alignment::Center)
            .spacing(5);

        let ui = widget_group(toggle_check).max_width(250);

        container(ui).center_x().center_y().into()
    }
}
