use iced::widget::column;
use iced::Element;
use iced::Sandbox;
use iced::Settings;
use iced::Size;

use sapphire_ui::widgets::container;
use sapphire_ui::widgets::progress_bar;
use sapphire_ui::widgets::slider;
use sapphire_ui::widgets::widget_group;

fn main() -> iced::Result {
    SliderApplication::run(Settings {
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
    SliderValueChanged(u8),
}

struct SliderApplication {
    value: u8,
}

impl Sandbox for SliderApplication {
    type Message = Message;

    fn new() -> Self {
        Self { value: 50 }
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

        let progress = widget_group(
            column![
                slider_val,
                progress_bar(0.0..=255.0, self.value.into()).width(200),
            ]
            .spacing(5),
        );

        container(progress).center_x().center_y().into()
    }
}
