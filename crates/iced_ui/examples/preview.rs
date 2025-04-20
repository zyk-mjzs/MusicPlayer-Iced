#![windows_subsystem = "windows"] // 在windows端隐藏控制台，不加会打开控制台
use std::borrow::Cow;

use iced::{Settings, Size, window};

use iced_font_awesome::{BRANDS_FONT_DATA, REGULAR_FONT_DATA, SOLID_FONT_DATA};
use iced_ui::Theme;

mod app;
use app::App;

pub fn main() -> iced::Result {
    iced::application("Iced Ui", App::update, App::view)
        .settings(Settings {
            default_font: Theme::default().font.regular,
            fonts: vec![
                Cow::Borrowed(BRANDS_FONT_DATA),
                Cow::Borrowed(REGULAR_FONT_DATA),
                Cow::Borrowed(SOLID_FONT_DATA),
            ],
            ..Settings::default()
        })
        .window(window::Settings {
            size: Size {
                width: 800.0,
                height: 600.0,
            },
            ..window::Settings::default()
        })
        .run()
}
