// https://docs.rs/quick-xml/latest/quick_xml/de/

pub mod xml_elements {
    pub enum RootElement {
        Arrangement,
        BoolParameter,
        Channel,
        EnumParameter,
        IntegerParameter,
        MetaData,
        Parameter,
        Project,
        RealParameter,
        Scene,
        TimeSignatureParameter,
        Track,
        Unknown,
    }

    impl From<String> for RootElement {
        fn from(s: String) -> Self {
            match s.as_str() {
                "Arrangement" => RootElement::Arrangement,
                "BoolParameter" => RootElement::BoolParameter,
                "Channel" => RootElement::Channel,
                "EnumParameter" => RootElement::EnumParameter,
                "IntegerParameter" => RootElement::IntegerParameter,
                "MetaData" => RootElement::MetaData,
                "Parameter" => RootElement::Parameter,
                "Project" => RootElement::Project,
                "RealParameter" => RootElement::RealParameter,
                "Scene" => RootElement::Scene,
                "TimeSignatureParameter" => RootElement::TimeSignatureParameter,
                "Track" => RootElement::Track,

                _ => RootElement::Unknown,
            }
        }
    }

    impl Into<String> for RootElement {
        fn into(self) -> String {
            match self {
                RootElement::Arrangement => "Arrangement".to_string(),
                RootElement::BoolParameter => "BoolParameter".to_string(),
                RootElement::Channel => "Channel".to_string(),
                RootElement::EnumParameter => "EnumParameter".to_string(),
                RootElement::IntegerParameter => "IntegerParameter".to_string(),
                RootElement::MetaData => "MetaData".to_string(),
                RootElement::Parameter => "Parameter".to_string(),
                RootElement::Project => "Project".to_string(),
                RootElement::RealParameter => "RealParameter".to_string(),
                RootElement::Scene => "Scene".to_string(),
                RootElement::TimeSignatureParameter => "TimeSignatureParameter".to_string(),
                RootElement::Track => "Track".to_string(),
                RootElement::Unknown => "Unknown".to_string(),
            }
        }
    }
}

#[cfg(test)]
#[test]

fn test() {
    use crate::xml_elements::xml_elements::RootElement;
    let b: String = RootElement::Project.into();
}
