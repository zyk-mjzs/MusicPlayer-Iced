use crate::theme::Theme;
use iced::{Element, Font, Length, widget::Text};
pub fn common_text(content: &str) -> CommonText {
    CommonText::new(content)
}

#[derive(Debug, Clone)]
pub struct CommonText<'a> {
    text: &'a str,
    size: u16,
    font: Font,
    width: Length,
    height: Length,
}

impl<'a> CommonText<'a> {
    pub fn new(text: &'a str) -> Self {
        let theme = Theme::default();
        Self {
            text,
            size: 13,
            font: theme.font.regular,
            width: Length::Shrink,
            height: Length::Shrink,
        }
    }

    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    pub fn size(mut self, size: u16) -> Self {
        self.size = size;
        self
    }
}

impl<'a, Message> From<CommonText<'a>> for Element<'a, Message> {
    fn from(common_text: CommonText<'a>) -> Element<'a, Message> {
        Text::new(common_text.text)
            .font(common_text.font)
            .size(common_text.size)
            .width(common_text.width)
            .height(common_text.height)
            .into()
    }
}
