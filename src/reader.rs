mod daw_project {

    use crate::xml_elements::XMLElements::RootElement;

    use std::io::BufReader;
    use std::{fs::File, path::PathBuf};

    use xml::reader::{EventReader, XmlEvent};

    pub fn read(path: &PathBuf) -> std::io::Result<()> {
        let file = File::open(path)?;
        let file = BufReader::new(file);

        let parser = EventReader::new(file);

        // Here we can parse each element
        for e in parser {
            // It parses
            match e {
                Ok(XmlEvent::StartElement { name, .. }) => match name.to_string().into() {
                    RootElement::Project => {
                        println!("{}", name)
                    }
                    RootElement::Parameter => {
                        println!("{}", name)
                    }
                    RootElement::Unknown => {}
                    RootElement::Arrangement => todo!(),
                    RootElement::BoolParameter => todo!(),
                    RootElement::Channel => todo!(),
                    RootElement::EnumParameter => todo!(),
                    RootElement::IntegerParameter => todo!(),
                    RootElement::MetaData => todo!(),
                    RootElement::RealParameter => todo!(),
                    RootElement::Scene => todo!(),
                    RootElement::TimeSignatureParameter => todo!(),
                    RootElement::Track => todo!(),
                },
                Ok(XmlEvent::EndElement { name }) => {
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
