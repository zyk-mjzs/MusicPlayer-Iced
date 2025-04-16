use crate::app::index::Message;
use iced::widget::{column, row, text};
use iced::{Element, Length, alignment};
use iced_ui::components::button::text_button;

pub fn button_page<'a>() -> Element<'a, Message> {
    column![
        text("Button Page")
            .size(24)
            .width(Length::Fill)
            .align_x(alignment::Horizontal::Center),
        row![
            text_button("text_button").on_press(Message::Press),
        ]
        .spacing(10)
    ]
    .padding(20)
    .spacing(20)
    .width(Length::Fill)
    .into()
}
