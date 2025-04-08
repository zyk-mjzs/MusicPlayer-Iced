mod music_player;

use music_player::{update, view};


pub fn main() -> iced::Result {
    iced::run("MusicPlayer", update, view)
}