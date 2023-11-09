pub mod main_structure {
    pub enum main_names {
        Project,
        Parameter,
        Unknown,
    }

    impl From<String> for main_names {
        fn from(s: String) -> Self {
            match s.as_str() {
                "Project" => main_names::Project,
                "Parameter" => main_names::Parameter,

                _ => main_names::Unknown,
            }
        }
    }

    impl Into<String> for main_names {
        fn into(self) -> String {
            match self {
                main_names::Project => "Project".to_string(),
                main_names::Parameter => "Parameter".to_string(),
                main_names::Unknown => "Unknown".to_string(),
            }
        }
    }
}

#[cfg(test)]
#[test]

fn test() {
    use crate::main_structure::main_structure::main_names;
    let b: String = main_names::Project.into();
}
