use iced::widget::{button, column, container, row, text};
use iced::{Element, Length, alignment};

use super::icon_page::icon_page;
use super::left_sidebar::left_sidebar;

#[derive(Default)]
pub struct App {
    current_page: Page,
}

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub enum Page {
    #[default]
    Text,
    Button,
    Progress,
    Icon,
}

#[derive(Debug, Clone)]
pub enum Message {
    Play,
    Stop,
    NavigateTo(Page),
}

impl App {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::NavigateTo(page) => {
                self.current_page = page;
            }
            _ => {}
        }
    }

    fn view_text_page(&self) -> Element<Message> {
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

    fn view_button_page(&self) -> Element<Message> {
        column![
            text("Button Page")
                .size(24)
                .width(Length::Fill)
                .align_x(alignment::Horizontal::Center),
            row![
                button("Click Me!").on_press(Message::Play),
                button("Another Button").on_press(Message::Stop)
            ]
            .spacing(10)
        ]
        .padding(20)
        .spacing(20)
        .width(Length::Fill)
        .into()
    }

    fn view_progress_page(&self) -> Element<Message> {
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

    fn view_icon_page(&self) -> Element<Message> {
        icon_page()
    }

    pub fn view(&self) -> Element<Message> {
        let page_content = match self.current_page {
            Page::Text => self.view_text_page(),
            Page::Button => self.view_button_page(),
            Page::Progress => self.view_progress_page(),
            Page::Icon => self.view_icon_page(),
        };

        let main_content = container(page_content)
            .width(Length::Fill)
            .align_x(alignment::Horizontal::Center);

        container(row![left_sidebar(), main_content])
            .width(Length::Fill)
            .height(Length::Fill)
            // .center_x()
            // .center_y()
            .into()
    }
}
