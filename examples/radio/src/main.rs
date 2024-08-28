use iced::widget::column;
use iced::widget::horizontal_space;
use iced::widget::row;
use iced::Element;
use iced::Length;
use iced::Sandbox;
use iced::Settings;
use iced::Size;

use sapphire_ui::widgets::container;
use sapphire_ui::widgets::radio;
use sapphire_ui::widgets::small_text;
use sapphire_ui::widgets::widget_group;

fn main() -> iced::Result {
    RadioApplication::run(Settings {
        window: iced::window::Settings {
            size: Size {
                width: 300.,
                height: 250.,
            },
            ..Default::default()
        },
        ..Default::default()
    })
}

struct RadioApplication {
    color: Option<Color>,
    layout: Layout,
}

#[derive(Clone, Copy, Debug)]
enum Message {
    LayoutChanged(Layout),
    ColorSelected(Color),
}

#[derive(Clone, Copy, Default, Debug, Eq, PartialEq)]
enum Layout {
    #[default]
    Row,
    Column,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

impl Sandbox for RadioApplication {
    type Message = Message;

    fn new() -> Self {
        Self {
            color: None,
            layout: Layout::default(),
        }
    }

    fn title(&self) -> String {
        format!("Radio example")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::LayoutChanged(layout) => self.layout = layout,
            Message::ColorSelected(color) => self.color = Some(color),
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let layout_row_radio = radio(
            "Row",
            Layout::Row,
            Some(self.layout),
            Message::LayoutChanged,
        );
        let layout_column_radio = radio(
            "Column",
            Layout::Column,
            Some(self.layout),
            Message::LayoutChanged,
        );

        let red_radio = radio("Red", Color::Red, self.color, Message::ColorSelected);
        let green_radio = radio("Green", Color::Green, self.color, Message::ColorSelected);
        let blue_radio = radio("Blue", Color::Blue, self.color, Message::ColorSelected);

        let layout: Element<_> = match self.layout {
            Layout::Row => row![layout_row_radio, horizontal_space(), layout_column_radio]
                .spacing(10)
                .width(Length::Fill)
                .into(),
            Layout::Column => column![layout_row_radio, layout_column_radio]
                .spacing(5)
                .into(),
        };

        let color = column![red_radio, green_radio, blue_radio].spacing(5);

        let layout_selection = column![small_text("Select layout type:"), layout,].spacing(5);
        let color_selection = column![
            small_text("Select color:"),
            widget_group(color).width(Length::Fill),
        ]
        .spacing(5);

        let ui = column![layout_selection, color_selection,]
            .spacing(10)
            .padding([10, 10])
            .width(200);

        container(ui).center_x().center_y().into()
    }
}
