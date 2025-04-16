use crate::theme::Theme;
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{button, text};
use iced::{Background, Border, Element, Length, Shadow};

pub fn confirm_button<Message>(content: &str) -> ConfirmButton<Message> {
    ConfirmButton::new(content)
}

#[derive(Debug, Clone)]
pub struct ConfirmButton<'a, Message> {
    text: &'a str,
    width: Length,
    height: Length,
    align_x: Horizontal,
    on_press: Option<Message>,
    theme: Theme,
}

impl<'a, Message> ConfirmButton<'a, Message> {
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            width: Length::Shrink,
            height: Length::Shrink,
            align_x: Horizontal::Center,
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

    pub fn align_x(mut self, align_x: Horizontal) -> Self {
        self.align_x = align_x;
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

impl<'a, Message> From<ConfirmButton<'a, Message>> for Element<'a, Message>
where
    Message: 'a + Clone,
{
    fn from(button: ConfirmButton<'a, Message>) -> Element<'a, Message> {
        let mut btn = button::Button::new(
            text(button.text)
                .font(button.theme.font.bold)
                .width(button.width)
                .height(button.height)
                .align_x(button.align_x)
                .align_y(Vertical::Center),
        )
        .style(move |_, is_hovered: button::Status| button::Style {
            background: match is_hovered {
                button::Status::Hovered => Some(Background::Color(
                    button.theme.color.primary.scale_alpha(0.7),
                )),
                _ => Some(Background::Color(button.theme.color.primary)),
            },
            text_color: button.theme.color.background,
            border: Border::default().rounded(8),
            shadow: Shadow::default(),
        });

        if let Some(msg) = button.on_press {
            btn = btn.on_press(msg);
        }

        btn.into()
    }
}
