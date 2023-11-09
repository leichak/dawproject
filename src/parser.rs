pub mod parser {

    use crate::xml_elements::xml_elements::RootElement;

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
                    RootElement::Arrangement => {}
                    RootElement::BoolParameter => {}
                    RootElement::Channel => {}
                    RootElement::EnumParameter => {}
                    RootElement::IntegerParameter => {}
                    RootElement::MetaData => {}
                    RootElement::RealParameter => {}
                    RootElement::Scene => {}
                    RootElement::TimeSignatureParameter => {}
                    RootElement::Track => {}
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
