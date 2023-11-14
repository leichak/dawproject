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

    #[derive(Deserialize, Debug)]
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
    enum TrackChannelEnum {
        Track(Track),
        Channel(Channel),
    }

    type TrackChannel = Vec<TrackChannelEnum>;

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
        #[serde(default)]
        params: TrackChannel,
        #[serde(rename = "$value")]
        #[serde(default)]
        contentType: Vec<ContentType>,
        #[serde(rename = "@loaded")]
        loaded: bool,
        #[serde(rename = "$value")]
        #[serde(default)]
        track_channel: Vec<TrackChannel>,
    }
    // https://rahul-thakoor.github.io/rust-raw-string-literals/ HASH IN Raw string literals, when it is r# # raw it does not use escape parameters
    let xml = r##"<Track contentType="notes" loaded="true" id="id2" name="Bass" color="#2312323" comment="dupa"></Track>"##;

    let mut obj: Track = from_str(xml).unwrap();

    println!("Deserialized object {:?} ", obj);
}

#[test]
fn parse_channel() {
    /*
    Channel Schema

    <xs:complexType name="channel">
      <xs:complexContent>
        <xs:extension base="lane">
          <xs:sequence>
            <xs:element name="Devices" minOccurs="0">
              <xs:complexType>
                <xs:sequence>
                  <xs:choice minOccurs="0" maxOccurs="unbounded">
                    <xs:element ref="Device"/>
                    <xs:element ref="Vst2Plugin"/>
                    <xs:element ref="Vst3Plugin"/>
                    <xs:element ref="ClapPlugin"/>
                    <xs:element ref="BuiltinDevice"/>
                    <xs:element ref="Equalizer"/>
                    <xs:element ref="Compressor"/>
                    <xs:element ref="NoiseGate"/>
                    <xs:element ref="Limiter"/>
                    <xs:element ref="AuPlugin"/>
                  </xs:choice>
                </xs:sequence>
              </xs:complexType>
            </xs:element>
            <xs:element name="Mute" type="boolParameter" minOccurs="0"/>
            <xs:element name="Pan" type="realParameter" minOccurs="0"/>
            <xs:element name="Sends" minOccurs="0">
              <xs:complexType>
                <xs:sequence>
                  <xs:element name="Send" type="send" minOccurs="0" maxOccurs="unbounded"/>
                </xs:sequence>
              </xs:complexType>
            </xs:element>
            <xs:element name="Volume" type="realParameter" minOccurs="0"/>
          </xs:sequence>
          <xs:attribute name="audioChannels" type="xs:int"/>
          <xs:attribute name="destination" type="xs:IDREF"/>
          <xs:attribute name="role" type="mixerRole"/>
          <xs:attribute name="solo" type="xs:boolean"/>
        </xs:extension>
      </xs:complexContent>
    </xs:complexType>
     */

    use crate::bool_parameter::BoolParameter;
    use crate::parameter::Parameter;
    use quick_xml::de::from_str;
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct FileReference {
        #[serde(rename = "@path")]
        path: String,
        #[serde(rename = "@external")]
        external: bool,
    }

    #[derive(Deserialize)]
    enum DeviceRole {
        instrument,
        noteFX,
        audioFX,
        analyzer,
    }

    #[derive(Deserialize)]
    struct Device {
        // Extends referenceable
        #[serde(rename = "@id")]
        id: String,
        // Extension end
        #[serde(rename = "Enabled")]
        enabled: BoolParameter,
        #[serde(rename = "@deviceRole")]
        device_role: DeviceRole,
        #[serde(rename = "@loaded")]
        loaded: bool,
        #[serde(rename = "@deviceName")]
        device_name: String,
        #[serde(rename = "@deviceID")]
        device_id: String,
        #[serde(rename = "@deviceVendor")]
        device_vendor: String,
        #[serde(rename = "State")]
        state: FileReference,
        #[serde(rename = "Parameters")]
        #[serde(default)]
        automated_parameters: Vec<Parameter>,
    }

    #[derive(Deserialize)]
    struct Plugin {
        // Extends device
        #[serde(rename = "@id")]
        id: String,
        #[serde(rename = "Enabled")]
        enabled: BoolParameter,
        #[serde(rename = "@deviceRole")]
        device_role: DeviceRole,
        #[serde(rename = "@loaded")]
        loaded: bool,
        #[serde(rename = "@deviceName")]
        device_name: String,
        #[serde(rename = "@deviceID")]
        device_id: String,
        #[serde(rename = "@deviceVendor")]
        device_vendor: String,
        #[serde(rename = "State")]
        state: FileReference,
        #[serde(rename = "Parameters")]
        #[serde(default)]
        automated_parameters: Vec<Parameter>,
        // End of extension
        #[serde(rename = "@pluginVersion")]
        plugin_version: String,
    }

    #[derive(Deserialize)]
    struct Vst2Plugin {
        // Extends plugin
        // Extends device
        #[serde(rename = "@id")]
        id: String,
        #[serde(rename = "Enabled")]
        enabled: BoolParameter,
        #[serde(rename = "@deviceRole")]
        device_role: DeviceRole,
        #[serde(rename = "@loaded")]
        loaded: bool,
        #[serde(rename = "@deviceName")]
        device_name: String,
        #[serde(rename = "@deviceID")]
        device_id: String,
        #[serde(rename = "@deviceVendor")]
        device_vendor: String,
        #[serde(rename = "State")]
        state: FileReference,
        #[serde(rename = "Parameters")]
        #[serde(default)]
        automated_parameters: Vec<Parameter>,
        // End of extension
        #[serde(rename = "@pluginVersion")]
        plugin_version: String,
    }

    #[derive(Deserialize)]
    struct Vst3Plugin {
        // Extends Plugin
        // Extends device
        #[serde(rename = "@id")]
        id: String,
        #[serde(rename = "Enabled")]
        enabled: BoolParameter,
        #[serde(rename = "@deviceRole")]
        device_role: DeviceRole,
        #[serde(rename = "@loaded")]
        loaded: bool,
        #[serde(rename = "@deviceName")]
        device_name: String,
        #[serde(rename = "@deviceID")]
        device_id: String,
        #[serde(rename = "@deviceVendor")]
        device_vendor: String,
        #[serde(rename = "State")]
        state: FileReference,
        #[serde(rename = "Parameters")]
        #[serde(default)]
        automated_parameters: Vec<Parameter>,
        // End of extension
        #[serde(rename = "@pluginVersion")]
        plugin_version: String,
    }

    #[derive(Deserialize)]
    struct ClapPlugin {
        // Extends Plugin
        // Extends device
        #[serde(rename = "@id")]
        id: String,
        #[serde(rename = "Enabled")]
        enabled: BoolParameter,
        #[serde(rename = "@deviceRole")]
        device_role: DeviceRole,
        #[serde(rename = "@loaded")]
        loaded: bool,
        #[serde(rename = "@deviceName")]
        device_name: String,
        #[serde(rename = "@deviceID")]
        device_id: String,
        #[serde(rename = "@deviceVendor")]
        device_vendor: String,
        #[serde(rename = "State")]
        state: FileReference,
        #[serde(rename = "Parameters")]
        #[serde(default)]
        automated_parameters: Vec<Parameter>,
        // End of extension
        #[serde(rename = "@pluginVersion")]
        plugin_version: String,
    }

    #[derive(Deserialize)]
    struct BuiltinDevice {
        // Extends device
        #[serde(rename = "@id")]
        id: String,
        #[serde(rename = "Enabled")]
        enabled: BoolParameter,
        #[serde(rename = "@deviceRole")]
        device_role: DeviceRole,
        #[serde(rename = "@loaded")]
        loaded: bool,
        #[serde(rename = "@deviceName")]
        device_name: String,
        #[serde(rename = "@deviceID")]
        device_id: String,
        #[serde(rename = "@deviceVendor")]
        device_vendor: String,
        #[serde(rename = "State")]
        state: FileReference,
        #[serde(rename = "Parameters")]
        #[serde(default)]
        automated_parameters: Vec<Parameter>,
    }

    #[derive(Deserialize)]
    struct Equalizer {
        // Extends builtinDevice
        #[serde(rename = "@id")]
        id: String,
        #[serde(rename = "Enabled")]
        enabled: BoolParameter,
        #[serde(rename = "@deviceRole")]
        device_role: DeviceRole,
        #[serde(rename = "@loaded")]
        loaded: bool,
        #[serde(rename = "@deviceName")]
        device_name: String,
        #[serde(rename = "@deviceID")]
        device_id: String,
        #[serde(rename = "@deviceVendor")]
        device_vendor: String,
        #[serde(rename = "State")]
        state: FileReference,
        #[serde(rename = "Parameters")]
        #[serde(default)]
        automated_parameters: Vec<Parameter>,
        // End of extension
        // Here vector of 3 elements
    }

    use crate::real_parameter::RealParameter;

    #[derive(Deserialize)]
    enum CompressorParamsEnum {
        Attack(RealParameter),
        AutoMakeup(BoolParameter),
        InputGain(RealParameter),
        OutputGain(RealParameter),
        Ratio(RealParameter),
        Release(RealParameter),
        Threshold(RealParameter),
    }

    type CompressorParams = Vec<CompressorParamsEnum>;

    #[derive(Deserialize)]
    struct Compressor {
        // Extendes builtInDevice
        #[serde(rename = "@id")]
        id: String,
        #[serde(rename = "Enabled")]
        enabled: BoolParameter,
        #[serde(rename = "@deviceRole")]
        device_role: DeviceRole,
        #[serde(rename = "@loaded")]
        loaded: bool,
        #[serde(rename = "@deviceName")]
        device_name: String,
        #[serde(rename = "@deviceID")]
        device_id: String,
        #[serde(rename = "@deviceVendor")]
        device_vendor: String,
        #[serde(rename = "State")]
        state: FileReference,
        #[serde(rename = "Parameters")]
        #[serde(default)]
        automated_parameters: Vec<Parameter>,
        // Extension ends
        #[serde(default)]
        params: CompressorParams,
    }

    #[derive(Deserialize)]
    enum NoiseGateParamsEnum {
        Attack(RealParameter),
        Range(RealParameter),
        Ratio(RealParameter),
        Release(RealParameter),
        Threshold(RealParameter),
    }

    type NoiseGateParams = Vec<NoiseGateParamsEnum>;

    #[derive(Deserialize)]
    struct NoiseGate {
        // Extendes builtInDevice
        #[serde(rename = "@id")]
        id: String,
        #[serde(rename = "Enabled")]
        enabled: BoolParameter,
        #[serde(rename = "@deviceRole")]
        device_role: DeviceRole,
        #[serde(rename = "@loaded")]
        loaded: bool,
        #[serde(rename = "@deviceName")]
        device_name: String,
        #[serde(rename = "@deviceID")]
        device_id: String,
        #[serde(rename = "@deviceVendor")]
        device_vendor: String,
        #[serde(rename = "State")]
        state: FileReference,
        #[serde(rename = "Parameters")]
        #[serde(default)]
        automated_parameters: Vec<Parameter>,
        #[serde(default)]
        params: NoiseGateParams,
    }

    #[derive(Deserialize)]
    enum LimiterParamsEnum {
        Attack(RealParameter),
        InputGain(RealParameter),
        OutputGain(RealParameter),
        Release(RealParameter),
        Threshold(RealParameter),
    }

    type LimiterParams = Vec<LimiterParamsEnum>;

    #[derive(Deserialize)]
    struct Limiter {
        // Extendes builtInDevice
        #[serde(rename = "@id")]
        id: String,
        #[serde(rename = "Enabled")]
        enabled: BoolParameter,
        #[serde(rename = "@deviceRole")]
        device_role: DeviceRole,
        #[serde(rename = "@loaded")]
        loaded: bool,
        #[serde(rename = "@deviceName")]
        device_name: String,
        #[serde(rename = "@deviceID")]
        device_id: String,
        #[serde(rename = "@deviceVendor")]
        device_vendor: String,
        #[serde(rename = "State")]
        state: FileReference,
        #[serde(rename = "Parameters")]
        #[serde(default)]
        automated_parameters: Vec<Parameter>,
        // Extension ends
        #[serde(default)]
        params: LimiterParams,
    }

    #[derive(Deserialize)]
    struct AuPlugin {
        // Extends Plugin
        #[serde(rename = "@id")]
        id: String,
        #[serde(rename = "Enabled")]
        enabled: BoolParameter,
        #[serde(rename = "@deviceRole")]
        device_role: DeviceRole,
        #[serde(rename = "@loaded")]
        loaded: bool,
        #[serde(rename = "@deviceName")]
        device_name: String,
        #[serde(rename = "@deviceID")]
        device_id: String,
        #[serde(rename = "@deviceVendor")]
        device_vendor: String,
        #[serde(rename = "State")]
        state: FileReference,
        #[serde(rename = "Parameters")]
        #[serde(default)]
        automated_parameters: Vec<Parameter>,
        #[serde(rename = "@pluginVersion")]
        plugin_version: String,
    }

    #[derive(Deserialize, Debug)]
    enum DeviceTypes {
        Device,
        Vst2Plugin,
        Vst3Plugin,
        ClapPlugin,
        BuiltinDevice,
        Equalizer,
        Compressor,
        NoiseGate,
        Limiter,
        AuPlugin,
    }

    #[derive(Deserialize, Debug)]
    struct Devices {
        #[serde(default)]
        #[serde(rename = "$value")]
        devices: Vec<DeviceTypes>,
    }

    #[derive(Deserialize)]
    enum SendParamsEnum {
        Pan(RealParameter),
        Volume(RealParameter),
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "lowercase")]
    enum SendTypeEnum {
        Pre,
        Post,
    }

    #[derive(Deserialize)]
    struct SendType {
        #[serde(rename = "$value")]
        field: Vec<SendTypeEnum>,
    }

    type SendParams = Vec<SendParamsEnum>;

    #[derive(Deserialize)]
    struct Send {
        // Extends referenceable
        #[serde(rename = "@id")]
        id: String,
        #[serde(default)]
        params: SendParams,
        #[serde(rename = "@destination")]
        destination: String,
        #[serde(rename = "@type")]
        send_type: SendType,
    }

    #[derive(Deserialize, Debug)]
    struct Sends {
        #[serde(default)]
        #[serde(rename = "$value")]
        devices: Vec<DeviceTypes>,
    }

    #[derive(Deserialize, Debug)]
    enum ChannelElementsEnum {
        Devices(Devices),
        Mute(BoolParameter),
        Pan(RealParameter),
        Sends(Sends),
    }

    type ChannelElements = Vec<ChannelElementsEnum>;

    #[derive(Deserialize, Debug)]
    #[serde(rename_all(deserialize = "lowercase"))]
    enum MixerRoleEnum {
        Regular,
        Master,
        Effect,
        SubMix,
        Vca,
    }

    #[derive(Deserialize, Debug)]
    struct MixerRole {
        #[serde(rename = "$value")]
        field: Vec<MixerRoleEnum>,
    }

    #[derive(Deserialize, Debug)]
    struct Channel {
        // Extends lane
        #[serde(rename = "@id")]
        id: String,
        #[serde(rename = "@name")]
        name: String, // attribute
        #[serde(rename = "@color")]
        color: String, // att
        #[serde(rename = "@comment")]
        comment: String, // att
        #[serde(default)]
        channel_elements: ChannelElements,
        #[serde(rename = "@audioChannels")]
        audio_channels: i32,
        #[serde(rename = "@destination")]
        destination: i32,
        #[serde(rename = "@role")]
        role: MixerRole,
        #[serde(rename = "@solo")]
        solo: bool,
    }

    let xml = r#"
    <Channel audioChannels="2" destination="id15" role="regular" solo="false" id="id3">
    <Devices>
      <ClapPlugin deviceID="org.surge-synth-team.surge-xt" deviceName="Surge XT" deviceRole="instrument" loaded="true" id="id7" name="Surge XT">
        <Parameters/>
        <Enabled value="true" id="id8" name="On/Off"/>
        <State path="plugins/d19b1f6e-bbb6-42fe-a6c9-54b41d97a05d.clap-preset"/>
      </ClapPlugin>
    </Devices>
    <Mute value="false" id="id6" name="Mute"/>
    <Pan max="1.000000" min="0.000000" unit="normalized" value="0.500000" id="id5" name="Pan"/>
    <Volume max="2.000000" min="0.000000" unit="linear" value="0.659140" id="id4" name="Volume"/>
  </Channel>"#;

    let mut obj: Channel = from_str(xml).unwrap();

    println!("Deserialized object {:?} ", obj);
}
