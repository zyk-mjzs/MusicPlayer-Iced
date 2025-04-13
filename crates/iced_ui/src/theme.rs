mod color;
mod font;

#[derive(Debug, Clone)]
pub struct Theme {
    pub color: color::Theme,
    pub font: font::Theme,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            color: color::Theme::default(),
            font: font::Theme::default(),
        }
    }
}

impl Theme {
    pub fn new() -> Self {
        Self::default()
    }
}