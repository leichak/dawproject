mod application;
mod arrangement;
mod bool_parameter;
mod channel;
mod content_type;
mod enum_parameter;
mod expression_type;
mod integer_parameter;
mod lane;
mod parameter;
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
static mut XML_ID: u32 = 0;

#[cfg(test)]
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

    use crate::parameter::Parameter;

    let xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
    <Project version = "1.0"><Application version = "2.0"></Application> </Project>"#;

    let mut obj: Parameter = from_str(xml).unwrap();

    println!("Deserialized object {:?} ", obj);
}

// Try to parse each complex type with appropriate xml within project

#[test]
fn parse_track() {
    /*
    Track schema

    <xs:complexType name="track">
    <xs:complexContent>
      <xs:extension base="lane">
        <xs:sequence>
          <xs:element ref="Channel" minOccurs="0"/>
          <xs:element ref="Track" minOccurs="0" maxOccurs="unbounded"/>
        </xs:sequence>
        <xs:attribute name="contentType">
          <xs:simpleType>
            <xs:list itemType="contentType"/>
          </xs:simpleType>
        </xs:attribute>
        <xs:attribute name="loaded" type="xs:boolean"/>
      </xs:extension>
     </xs:complexContent>
    </xs:complexType>
     */

    use content_type::ContentType;
    use quick_xml::de::from_str;
    use serde::Deserialize;

    #[derive(Deserialize)]
    enum MixerRole {
        regular,
        master,
        effecttrack,
        submix,
        vca,
    }

    #[derive(Deserialize)]
    pub struct Channel {
        // Extends lane
        #[serde(rename = "@id")]
        id: String,
        #[serde(rename = "@name")]
        name: String, // attribute
        #[serde(rename = "@color")]
        color: String, // att
        #[serde(rename = "@comment")]
        comment: String, // att
    }

    #[derive(Deserialize, Debug)]
    enum TrackChannel {
        Track,
        Channel,
    }

    #[derive(Deserialize, Debug)]
    struct Track {
        // Extends lane
        #[serde(rename = "@id")]
        id: String,
        #[serde(rename = "@name")]
        name: String, // attribute
        #[serde(rename = "@color")]
        color: String, // att
        #[serde(rename = "@comment")]
        comment: String, // att
        #[serde(rename = "$value")]
        #[serde(default)]
        contentType: Vec<ContentType>,
        #[serde(rename = "@loaded")]
        loaded: bool,
        #[serde(rename = "$value")]
        #[serde(default)]
        track_channel: Vec<TrackChannel>,
    }

    let xml = r#"<Track contentType="notes" loaded="true" id="id2" name="Bass" color="2312323" comment="dupa"></Track>"#;

    let mut obj: Track = from_str(xml).unwrap();

    println!("Deserialized object {:?} ", obj);
}
