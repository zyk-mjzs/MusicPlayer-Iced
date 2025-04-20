use crate::theme::Theme;
use iced::widget::{canvas, mouse_area};
use iced::{Element, Length, Point, Rectangle, Size};
use iced::mouse::Interaction;
use iced::event::Status;

pub fn common_slider<Message>(value: &f32) -> CommonSlider<Message> {
    CommonSlider::new(value)
}

#[derive(Debug, Clone)]
pub struct CommonSlider<'a, Message> {
    value: &'a f32,
    on_change: Option<Message>,
    theme: Theme,
}

impl<'a, Message> CommonSlider<'a, Message> {
    pub fn new(value: &'a f32) -> Self {
        Self {
            value,
            on_change: None,
            theme: Theme::default(),
        }
    }
    pub fn on_change(self, on_change: Message) -> Self {
        Self {
            on_change: Some(on_change),
            ..self
        }
    }
}

impl<'a, Message> From<CommonSlider<'a, Message>> for Element<'a, Message>
where
    Message: 'a + Clone,
{
    fn from(slider: CommonSlider<'a, Message>) -> Element<'a, Message> {
        let area = mouse_area(
            canvas(|size, cursor| {
                let value = *slider.value;
                let is_dragging = matches!(cursor, Interaction::Pointer);
                
                let track_height = 4.0;
                let thumb_radius = 8.0;
                let track_width = size.width - thumb_radius * 2.0;
                let track_y = size.height / 2.0 - track_height / 2.0;
                
                let thumb_x = thumb_radius + value * track_width;
                let thumb_y = size.height / 2.0;
                
                let mut frame = canvas::Frame::new(size, bounds.size());
                
                // Draw track
                frame.fill_rectangle(
                    Point::new(thumb_radius, track_y),
                    Size::new(track_width, track_height),
                    slider.theme.color.primary,
                );
                
                // Draw thumb
                frame.fill_circle(
                    Point::new(thumb_x, thumb_y),
                    thumb_radius,
                    slider.theme.color.primary,
                );
                
                if is_dragging {
                    if let Some(pos) = cursor.position() {
                        let new_value = ((pos.x - thumb_radius) / track_width)
                            .max(0.0)
                            .min(1.0);
                        
                        if let Some(on_change) = &slider.on_change {
                            return Status::Captured(on_change.clone());
                        }
                    }
                }
                
                Status::Ignored
            })
            .width(Length::Fill)
            .height(Length::Fixed(24.0))
        );

        area.into()
    }
}