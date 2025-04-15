use iced::widget::{column, container, text};
use iced::{Element, Length, alignment, alignment::Horizontal};

use iced_ui::button::text_button;

use super::index::{Message, Page};

pub fn left_sidebar() -> Element<'static, Message> {
    let sidebar_title = text("Navigation")
        .size(24)
        .width(Length::Fill)
        .align_x(Horizontal::Center);

    let sidebar_items = column![
        text_button("Text")
            .width(Length::Fill)
            .height(Length::from(50))
            .align_x(Horizontal::Left)
            .on_press(Message::NavigateTo(Page::Text)),
        text_button("Button")
            .width(Length::Fill)
            .height(Length::from(50))
            .align_x(Horizontal::Left)
            .on_press(Message::NavigateTo(Page::Button)),
        text_button("Progress")
            .width(Length::Fill)
            .height(Length::from(50))
            .align_x(Horizontal::Left)
            .on_press(Message::NavigateTo(Page::Progress))
    ]
    .width(Length::Fill)
    .align_x(Horizontal::Left);

    container(
        column![sidebar_title, sidebar_items]
            .padding(15)
            .spacing(20)
            .width(200),
    )
    .height(Length::Fill)
    .style(container::rounded_box)
    .into()
}
