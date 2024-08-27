use iced::Element;
use iced::Sandbox;
use iced::Settings;
use iced::Size;

use sapphire_ui::widgets::checkbox;
use sapphire_ui::widgets::container;
use sapphire_ui::widgets::widget_group;

fn main() -> iced::Result {
    ChangeThemeApplication::run(Settings {
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
    ChangeStyle(bool),
}

struct ChangeThemeApplication {
    use_dark_style: bool,
}

impl Sandbox for ChangeThemeApplication {
    type Message = Message;

    fn new() -> Self {
        Self {
            use_dark_style: false,
        }
    }

    fn title(&self) -> String {
        format!("Theme example")
    }

    fn theme(&self) -> iced::Theme {
        if self.use_dark_style {
            iced::Theme::Dark
        } else {
            iced::Theme::Light
        }
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::ChangeStyle(use_dark) => self.use_dark_style = use_dark,
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let check = checkbox("Use dark style", self.use_dark_style).on_toggle(Message::ChangeStyle);
        let ui = widget_group(check);

        container(ui).center_x().center_y().into()
    }
}
