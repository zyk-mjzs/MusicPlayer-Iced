use crate::app::index::Message;
use iced::widget::{column, text};
use iced::{Element, Length, alignment};

pub fn text_page<'a>() -> Element<'a, Message> {
    column![
        text("Text Page")
            .size(24)
            .width(Length::Fill)
            .align_x(alignment::Horizontal::Center),
        text("This is the text content example.")
            .size(16)
            .width(Length::Fill)
    ]
    .padding(20)
    .spacing(20)
    .width(Length::Fill)
    .into()
}
