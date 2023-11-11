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

#[cfg(test)]
#[test]
fn test_parsing() {
    use std::path::PathBuf;
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("resources/tests/project.xml");

    let ok = parser::parser::read(&d);
}
