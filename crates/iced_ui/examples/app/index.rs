use iced::widget::{container, row};
use iced::{Element, Length, alignment};

use super::left_sidebar::left_sidebar;
use super::pages::{button_page, icon_page, progress_page, text_page};

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
    Press,
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

    pub fn view(&self) -> Element<Message> {
        let page_content = match self.current_page {
            Page::Text => text_page(),
            Page::Button => button_page(),
            Page::Progress => progress_page(),
            Page::Icon => icon_page(),
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
