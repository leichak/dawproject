mod main_structure;

mod daw_project {

    use crate::main_structure::main_structure::main_names;

    use std::io::BufReader;
    use std::{fs::File, path::PathBuf};

    use xml::reader::{EventReader, XmlEvent};

    pub fn read(path: &PathBuf) -> std::io::Result<()> {
        let file = File::open(path)?;
        let file = BufReader::new(file);

        let parser = EventReader::new(file);
        let mut depth = 0;

        // Here we can parse each element
        for e in parser {
            match e {
                Ok(XmlEvent::StartElement { name, .. }) => {
                    let result: main_names = name.to_string().into();

                    match name.to_string().into() {
                        main_names::Project => {
                            println!("{}", name)
                        }
                        main_names::Parameter => {
                            println!("{}", name)
                        }
                        main_names::Unknown => {}
                    }
                }
                Ok(XmlEvent::EndElement { name }) => {
                    // depth -= 1;
                    println!("{:spaces$}-{name}", "", spaces = 1);
                }
                Err(e) => {
                    eprintln!("Error: {e}");
                    break;
                }
                _ => {}
            }
        }

        Ok(())
    }
}

#[cfg(test)]
#[test]
fn it_works() {
    use std::path::PathBuf;
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("resources/tests/project.xml");

    let ok = daw_project::read(&d);
}
