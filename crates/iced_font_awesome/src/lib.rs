use iced::{font::{load, Family}, Font};

mod fa;
pub use fa::*;

const REGULAR_FONT_DATA: &[u8] =
    include_bytes!("../assets/fontawesome-free-6.7.2-web/webfonts/fa-regular-400.ttf");

const BRANDS_FONT_DATA: &[u8] =
    include_bytes!("../assets/fontawesome-free-6.7.2-web/webfonts/fa-brands-400.ttf");

const SOLID_FONT_DATA: &[u8] =
    include_bytes!("../assets/fontawesome-free-6.7.2-web/webfonts/fa-solid-900.ttf");


pub fn load_font_awesome() {
    let _ = load(REGULAR_FONT_DATA);
    let _ = load(SOLID_FONT_DATA);
    let _ = load(BRANDS_FONT_DATA);
}
