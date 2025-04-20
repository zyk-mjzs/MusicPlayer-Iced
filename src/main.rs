#![windows_subsystem = "windows"] // 在windows端隐藏控制台，不加会打开控制台

mod music_player;

use iced::{Settings, Size, window};
use music_player::MusicPlayer;
use iced_ui::Theme;

pub fn main() -> iced::Result {
    iced::application("MusicPlayer", MusicPlayer::update, MusicPlayer::view)
        .settings(Settings {
            default_font: Theme::default().font.regular,
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
