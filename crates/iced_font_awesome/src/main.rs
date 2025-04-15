extern crate serde_json;
use std::fs::File;
use std::io::Write;
use serde_json::Value;

fn generate_icon_file(icon_type: &str, json: &Value) {
    let mut output = String::new();
    let enum_name = match icon_type {
        "regular" => "FontAwesomeRegular",
        "solid" => "FontAwesomeSolid",
        "brands" => "FontAwesomeBrands",
        _ => panic!("Invalid icon type"),
    };

    output.push_str(&format!("use iced::{{Font, font::Family}};\n\n"));
    output.push_str(&format!("use iced::widget::{{Text, text}};\n\n"));
    output.push_str(&format!("pub enum {} {{\n", enum_name));

    if let Value::Object(map) = json {
        for (key, value) in map {
            if let Some(free) = value.get("free") {
                if free.as_array().unwrap().contains(&Value::String(icon_type.to_string())) {
                    let _unicode = value.get("unicode").unwrap().as_str().unwrap();
                    if !key.chars().next().unwrap().is_numeric() {
                        let formatted_key = key.split('-').map(|s| {
                            let mut c = s.chars();
                            match c.next() {
                                None => String::new(),
                                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                            }
                        }).collect::<Vec<String>>().join("");
                        output.push_str(&format!("    {},\n", formatted_key));
                    }
                }
            }
        }
    }

    output.push_str("}\n\n");
    output.push_str(&format!("impl {} {{\n", enum_name));
    output.push_str("    pub fn to_string(&self) -> &str {\n");
    output.push_str("        match self {\n");

    if let Value::Object(map) = json {
        for (key, value) in map {
            if let Some(free) = value.get("free") {
                if free.as_array().unwrap().contains(&Value::String(icon_type.to_string())) {
                    let unicode = value.get("unicode").unwrap().as_str().unwrap();
                    if !key.chars().next().unwrap().is_numeric() {
                        let formatted_key = key.split('-').map(|s| {
                            let mut c = s.chars();
                            match c.next() {
                                None => String::new(),
                                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                            }
                        }).collect::<Vec<String>>().join("");
                        output.push_str(&format!("            Self::{} => \"\\u{{{}}}\",\n", formatted_key, unicode));
                    }
                }
            }
        }
    }

    output.push_str("        }\n");
    output.push_str("    }\n");
    output.push_str("}\n\n");
    output.push_str(&format!("pub fn {}<'a>(c: {}) -> Text<'a> {{\n", icon_type, enum_name));
    output.push_str("    text(c.to_string().to_owned()).font(FONT)\n");
    output.push_str("}\n\n");
    output.push_str("const FONT: Font = Font {\n");
    output.push_str(&format!("    family: Family::Name(\"Font Awesome 6 {}\"),\n",
        match icon_type {
            "regular" => "Free",
            "solid" => "Free",
            "brands" => "Brands",
            _ => panic!("Invalid icon type"),
        }
    ));
    output.push_str(&format!("    weight: iced::font::Weight::{},\n", 
        match icon_type {
            "regular" => "Normal",
            "solid" => "Black",
            "brands" => "Normal",
            _ => panic!("Invalid icon type"),
        }
    ));
    output.push_str("    ..Font::DEFAULT\n");
    output.push_str("};\n");

    std::fs::create_dir_all("src/fa").expect("Unable to create directory");
    let file_path = format!("src/fa/{}.rs", icon_type);
    let mut file = File::create(&file_path).expect("Unable to create file");
    file.write_all(output.as_bytes()).expect("Unable to write data");
}

fn main() {
    let file_path = "assets/fontawesome-free-6.7.2-web/metadata/icons.json";
    let file = File::open(file_path).expect("Unable to open file");
    let json: Value = serde_json::from_reader(file).expect("Unable to parse JSON");

    generate_icon_file("regular", &json);
    generate_icon_file("solid", &json);
    generate_icon_file("brands", &json);
}