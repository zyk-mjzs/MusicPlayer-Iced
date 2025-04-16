mod fa;
pub use fa::*;

pub const REGULAR_FONT_DATA: &[u8] =
    include_bytes!("../assets/fontawesome-free-6.7.2-web/webfonts/fa-regular-400.ttf");

pub const BRANDS_FONT_DATA: &[u8] =
    include_bytes!("../assets/fontawesome-free-6.7.2-web/webfonts/fa-brands-400.ttf");

pub const SOLID_FONT_DATA: &[u8] =
    include_bytes!("../assets/fontawesome-free-6.7.2-web/webfonts/fa-solid-900.ttf");
