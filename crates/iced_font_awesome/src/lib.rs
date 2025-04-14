use iced::{font::{load, Family}, Font};

pub mod fa;

const REGULAR_FONT_DATA: &[u8] =
    include_bytes!("../assets/fontawesome-free-6.7.2-web/webfonts/fa-regular-400.ttf");

const BRANDS_FONT_DATA: &[u8] =
    include_bytes!("../assets/fontawesome-free-6.7.2-web/webfonts/fa-brands-400.ttf");

const SOLID_FONT_DATA: &[u8] =
    include_bytes!("../assets/fontawesome-free-6.7.2-web/webfonts/fa-solid-900.ttf");

const REGULAR_FONT: Font = Font {
    family: Family::Name("Font Awesome 6 Free"),
    ..Font::DEFAULT
};



const BRANDS_FONT: Font = Font {
    family: Family::Name("Font Awesome 6 Brands"),
    ..Font::DEFAULT
};

pub fn load_font_awesome() {
    load(REGULAR_FONT_DATA);
    load(SOLID_FONT_DATA);
    load(BRANDS_FONT_DATA);
}
