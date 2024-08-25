use iced::widget::row;
use iced::Alignment;
use iced::Element;
use iced::Sandbox;
use iced::Settings;
use iced::Size;

use sapphire_ui::widgets::container;
use sapphire_ui::widgets::slider;
use sapphire_ui::widgets::text;
use sapphire_ui::widgets::widget_group;

fn main() -> iced::Result {
    SliderApplication::run(Settings {
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

#[derive(Debug, Clone, Copy)]
enum Message {
    SliderValueChanged(u8),
}

struct SliderApplication {
    value: u8,
}

impl Sandbox for SliderApplication {
    type Message = Message;

    fn new() -> Self {
        Self { value: 0 }
    }

    fn title(&self) -> String {
        format!("Slider example")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::SliderValueChanged(value) => self.value = value,
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let slider_val = slider(0..=255, self.value, Message::SliderValueChanged).width(200);
        let slider_txt = text(format!("{:03}", self.value));

        let slider_row = row![slider_val, slider_txt]
            .align_items(Alignment::Center)
            .spacing(5);
        let ui = widget_group(slider_row);

        container(ui).center_x().center_y().into()
    }
}
