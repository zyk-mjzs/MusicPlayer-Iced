use iced::Font;
use iced::font::{Family, Weight};

#[derive(Debug, Clone)]
pub struct Theme {
    pub family: Family,
    pub regular: Font,
    pub bold: Font,
}

impl Default for Theme {
    fn default() -> Self {
        #[cfg(target_os = "macos")]
        let family_name = "PingFang SC";
        #[cfg(target_os = "windows")]
        let family_name = "Microsoft YaHei";
        #[cfg(not(any(target_os = "macos", target_os = "windows")))]
        let family_name = "Arial";

        let family = Family::Name(family_name);
        Self {
            family,
            regular: Font {
                family,
                weight: Weight::Normal,
                ..Default::default()
            },
            bold: Font {
                family,
                weight: Weight::Bold,
                ..Default::default()
            },
        }
    }
}
