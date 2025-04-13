
use iced::Color;

#[derive(Debug, Clone, Copy)]
pub struct Theme {
    pub primary: Color,
    pub background: Color,
    pub secondary: Color,
    pub accent: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            primary: Color::from_rgb8(52,58,64),
            secondary: Color::from_rgb8(155,155,155),
            accent: Color::from_rgb8(0,123,255),
            background: Color::from_rgb8(249,249,249),
        }
    }
}

impl Theme {
    pub fn new() -> Self {
        Self::default()
    }
}