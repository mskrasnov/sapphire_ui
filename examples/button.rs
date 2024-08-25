use iced::Size;
use sapphire_ui::theme::prelude::*;
use sapphire_ui::widgets::button;
use sapphire_ui::widgets::checkbox;
use sapphire_ui::widgets::container;
use sapphire_ui::widgets::scrollable;
use sapphire_ui::widgets::small_text;
use sapphire_ui::widgets::text;
use sapphire_ui::widgets::text_input;
use sapphire_ui::widgets::title_text;

use iced::widget::column;
use iced::widget::row;
// use iced::widget::text;
use iced::{Font, Sandbox, Settings};
use widget::horizontal_space;
use widget::vertical_space;

fn main() -> iced::Result {
    let settings = Settings {
        default_text_size: Pixels(THEME.global.text.size),
        default_font: Font::with_name(&THEME.global.text.font),
        window: iced::window::Settings {
            size: Size::new(250., 200.),
            resizable: false,
            ..iced::window::Settings::default()
        },
        ..Default::default()
    };
    ButtonExample::run(settings)
}

#[derive(Clone, Debug)]
enum Message {
    ButtonPressed,
    ValueChanged(String),
    CheckboxToggled(bool),
}

struct ButtonExample {
    label: String,
    value: String,
    pressed: bool,
}

impl Sandbox for ButtonExample {
    type Message = Message;

    fn new() -> Self {
        Self {
            label: "Press me!".to_string(),
            value: "".to_string(),
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
            Message::ValueChanged(value) => self.value = value,
            Message::CheckboxToggled(toggle) => self.pressed = toggle,
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let mut btn = button(text(&self.label)).on_press(Message::ButtonPressed);
        if self.pressed {
            btn = button(text(&self.label));
        }

        let txt_inpt =
            text_input("Put your text here...", &self.value).on_input(Message::ValueChanged);

        let space = vertical_space().height(Length::Fixed(1000.));
        let space1 = vertical_space().height(Length::Fixed(1000.));
        let clmn = column![
            title_text("Button example"),
            txt_inpt,
            text("Scroll me!"),
            row![space].spacing(10).width(Length::Fill),
            row![
                horizontal_space().width(Length::Fill),
                btn,
                checkbox("Press me", self.pressed).on_toggle(Message::CheckboxToggled),
                horizontal_space().width(Length::Fill),
            ]
            .align_items(iced::Alignment::Center)
            .spacing(5),
            space1,
            small_text("Amazing!"),
        ]
        .padding([10, 20])
        .spacing(10);

        container(scrollable(clmn)).into()
    }
}
