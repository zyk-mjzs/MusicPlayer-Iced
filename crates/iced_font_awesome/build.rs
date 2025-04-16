use heck::AsUpperCamelCase;
#[derive(serde::Serialize, serde::Deserialize, Clone)]
struct IconsConfig {
    pub unicode: String,
    pub styles: Vec<String>,
    pub key: Option<String>,
}
fn generate_icon_config() {
    let config: String =
        std::fs::read_to_string("assets/fontawesome-free-6.7.2-web/metadata/icons.json").unwrap();
    let config: serde_json::Value = serde_json::from_str(&config).unwrap();
    let icons_config_map = config.as_object().unwrap();
    let icons_config_list: Vec<IconsConfig> = icons_config_map
        .iter()
        .map(|(key, value)| {
            let mut icon_config: IconsConfig = serde_json::from_value(value.clone()).unwrap();
            icon_config.key = Some(key.clone());
            icon_config
        })
        .collect();
    let grouped_icons: std::collections::HashMap<String, Vec<IconsConfig>> = icons_config_list
        .into_iter()
        .fold(std::collections::HashMap::new(), |mut acc, icon| {
            for style in &icon.styles {
                acc.entry(style.clone()).or_default().push(icon.clone());
            }
            acc
        });
    let mod_rs_base_path = "src/fa";
    let mut mod_rs_output = String::new();
    let mod_rs_output_path = std::path::Path::new("src/fa.rs");
    std::fs::create_dir_all(mod_rs_base_path).unwrap();
    mod_rs_output.push_str(
        r#"#[macro_export]
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
"#,
    );

    for (style, icons) in grouped_icons {
        mod_rs_output.push_str(&format!("\nmod {};\n", style));
        mod_rs_output.push_str(&format!("\npub use {}::*;\n", style));
        let lib_output_path_str = format!("{}/{}.rs", mod_rs_base_path, style);
        let lib_output_path = std::path::Path::new(&lib_output_path_str);
        let mut lib_output = String::new();
        lib_output.push_str("use crate::font_awesome_formats;\n");
        lib_output.push_str(
            r#"
use iced::{Font, font::Family};
use iced::widget::{Text, text};
"#,
        );
        lib_output.push_str("\nfont_awesome_formats!{\n");
        let ident = AsUpperCamelCase(format!("FontAwesome_{},", style)).to_string();
        lib_output.push_str(&format!(" {},\n", ident));

        for icon in icons {
            let name = icon.key.clone().unwrap();
            if name.parse::<u64>().is_ok() {
                lib_output.push_str(&format!(
                    " {} => \"\\u{{{}}}\",\n",
                    AsUpperCamelCase(format!("Number{name}")),
                    icon.unicode.clone()
                ));
            } else if name[0..1].parse::<u64>().is_ok() {
                lib_output.push_str(&format!(
                    " {} => \"\\u{{{}}}\",\n",
                    AsUpperCamelCase(format!("Orther{name}")),
                    icon.unicode.clone()
                ));
            } else {
                lib_output.push_str(&format!(
                    " {} => \"\\u{{{}}}\",\n",
                    AsUpperCamelCase(name),
                    icon.unicode.clone()
                ));
            }
        }
        lib_output.push('}');
        lib_output.push_str(&format!(
            r#"
pub fn {style}<'a>(c: {ident}) -> Text<'a> {{
  text(c.to_str().to_owned()).font(FONT)
}}
const FONT: Font = Font {{
"#
        ));
        if style == "brands" {
            lib_output.push_str(r#"  family: Family::Name("Font Awesome 6 Brands"),"#);
        } else {
            lib_output.push_str(r#"  family: Family::Name("Font Awesome 6 Free"),"#);
        }
        if style == "solid" {
            lib_output.push_str("\n  weight: iced::font::Weight::Black,");
        }
        lib_output.push_str(r#"
  ..Font::DEFAULT
};
"#);
        std::fs::write(lib_output_path, lib_output).unwrap();
    }
    // let output_path = std::path::Path::new("src/icons.rs");
    std::fs::write(mod_rs_output_path, mod_rs_output).unwrap();
}

fn main() {
    // This build script is used to generate the font files for Font Awesome
    // The font files are generated from the Font Awesome website
    // The font files are then used in the iced application

    // The font files are generated from the Font Awesome website
    // The font files are then used in the iced application

    // The font files are generated from the Font Awesome website
    // The font files are then used in the iced application
    generate_icon_config();
    println!("cargo:rerun-if-changed=assets");
}
