mod parser;
mod xml_elements;

#[cfg(test)]
#[test]
fn test_parsing() {
    use std::path::PathBuf;
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("resources/tests/project.xml");

    let ok = parser::parser::read(&d);
}
