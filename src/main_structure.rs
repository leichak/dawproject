pub mod main_structure {
    pub enum RootElement {
        Project,
        Parameter,
        Unknown,
    }

    impl From<String> for RootElement {
        fn from(s: String) -> Self {
            match s.as_str() {
                "Project" => RootElement::Project,
                "Parameter" => RootElement::Parameter,

                _ => RootElement::Unknown,
            }
        }
    }

    impl Into<String> for RootElement {
        fn into(self) -> String {
            match self {
                RootElement::Project => "Project".to_string(),
                RootElement::Parameter => "Parameter".to_string(),
                RootElement::Unknown => "Unknown".to_string(),
            }
        }
    }
}

#[cfg(test)]
#[test]

fn test() {
    use crate::main_structure::main_structure::RootElement;
    let b: String = RootElement::Project.into();
}
