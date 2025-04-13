use crate::theme::Theme;
use iced::widget::{button, text};
use iced::{Background, Border, Element, Length, Shadow};

pub fn text_button<Message>(content: &str) -> TextButton<Message> {
    TextButton::new(content)
}

#[derive(Debug, Clone)]
pub struct TextButton<'a, Message> {
    text: &'a str,
    width: Length,
    height: Length,
    on_press: Option<Message>,
    theme: Theme,
}

impl<'a, Message> TextButton<'a, Message> {
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            width: Length::Shrink,
            height: Length::Shrink,
            on_press: None,
            theme: Theme::default(),
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

    pub fn on_press(mut self, msg: Message) -> Self {
        self.on_press = Some(msg);
        self
    }

    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
}

impl<'a, Message> From<TextButton<'a, Message>> for Element<'a, Message>
where
    Message: 'a + Clone,
{
    fn from(button: TextButton<'a, Message>) -> Element<'a, Message> {
        let mut btn = button::Button::new(
            text(button.text)
                .font(button.theme.font.bold)
                .center(),
        )
        .style(move |_, is_hovered: button::Status| button::Style {
            background: match is_hovered {
                button::Status::Hovered => {
                    Some(Background::Color(button.theme.color.primary.scale_alpha(0.1)))
                }
                _ => None,
            },
            text_color: button.theme.color.primary,
            border: Border::default().rounded(8),
            shadow: Shadow::default(),
        });

        if let Some(msg) = button.on_press {
            btn = btn.on_press(msg);
        }

        btn.width(button.width).height(button.height).into()
    }
}
