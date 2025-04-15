#![windows_subsystem = "windows"] // 在windows端隐藏控制台，不加会打开控制台
use iced::{Font, Settings, Size, window};

mod app;
use app::App;

pub fn main() -> iced::Result {
    iced::application("Iced Ui", App::update, App::view)
        .settings(Settings {
            default_font: Font::with_name("PingFang SC"),
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
