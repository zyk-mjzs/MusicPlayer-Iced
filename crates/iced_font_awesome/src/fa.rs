#[macro_export]
macro_rules! font_awesome_formats {
    ($ident: ident, $($name:ident => $unicode:expr),* $(,)?) => {
        #[derive(Debug, strum_macros::EnumIter)]
        pub enum $ident {
            $($name),*
        }
        impl $ident {
            pub fn to_str(&self) -> &'static str {
                match self {
                    $(Self::$name => $unicode),*
                }
            }
        }
    };
}

mod solid;

pub use solid::*;

mod brands;

pub use brands::*;

mod regular;

pub use regular::*;
