use iced::widget::{column, text};
use iced::{Element, Length, alignment};
use crate::app::index::Message;

pub fn progress_page<'a>() -> Element<'a, Message> {
  column![
      text("Progress Page")
          .size(24)
          .width(Length::Fill)
          .align_x(alignment::Horizontal::Center),
      text("Progress content will be added here.")
          .size(16)
          .width(Length::Fill)
  ]
  .padding(20)
  .spacing(20)
  .width(Length::Fill)
  .into()
}