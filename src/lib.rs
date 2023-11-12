mod application;
mod arrangement;
mod bool_parameter;
mod channel;
mod enum_parameter;
mod expression_type;
mod integer_parameter;
mod lane;
mod parameter;
mod parser;
mod project;
mod real_parameter;
mod scene;
mod time_signature_parameter;
mod timeline;
mod track;
mod transport;
mod unit;
mod xml_elements;

pub use serde::{Deserialize, Serialize};
static mut ID: u32 = 0;

#[cfg(test)]
#[test]
fn test_parsing() {
    use std::path::PathBuf;
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("resources/tests/project.xml");

    let ok: Result<(), std::io::Error> = parser::parser::read(&d);
}

#[test]
fn parse_project() {
    use quick_xml::de::from_str;
    use quick_xml::events::Event;
    use quick_xml::reader::Reader;

    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Debug)]
    struct Application {
        #[serde(rename = "@version")]
        version: f32,
    }

    #[derive(Deserialize, Debug)]
    struct Project {
        #[serde(rename = "@version")]
        version: f32,
        #[serde(rename = "Application")]
        application: Application,
    }

    let xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
    <Project version = "1.0"><Application version = "2.0"></Application> </Project>"#;

    let mut obj: Project = from_str(xml).unwrap();

    println!("Deserialized object {:?} ", obj);
}

#[test]
fn deal_with_abstract_types() {
    use quick_xml::de::from_str;
    use quick_xml::events::Event;
    use quick_xml::reader::Reader;

    use serde::{Deserialize, Serialize};

    // Maybe we should manually go through the XML and then check opening and enclosing events
    // Maybe abstraction does not matter after all and we can parse it as it is - having derived fields within struct
}
