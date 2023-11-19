mod application;
mod arrangement;
mod bool_parameter;
mod channel;
mod content_type;
mod device;
mod enum_parameter;
mod expression_type;
mod file_reference;
mod integer_parameter;
mod lane;
mod mixer_role;
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

use quick_xml::de::from_str;
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

    use crate::channel::Channel;
    use content_type::ContentType;
    use mixer_role::MixerRoleEnum;
    use quick_xml::de::from_str;
    use serde::Deserialize;

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
        name: Option<String>,
        #[serde(rename = "@color")]
        color: Option<String>, // att
        #[serde(rename = "@comment")]
        comment: Option<String>, // att
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
    let xml = r##"
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
  </Channel>"##;
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

    #[derive(Deserialize, Debug)]
    struct FileReference {
        #[serde(rename = "@path")]
        path: String,
        #[serde(rename = "@external")]
        external: Option<bool>,
    }

    #[derive(Deserialize, Debug)]
    enum DeviceRole {
        instrument,
        noteFX,
        audioFX,
        analyzer,
    }

    #[derive(Deserialize, Debug)]
    enum DeviceElementsEnum {
        Parameters,
        Enabled(BoolParameter),
        State(FileReference),
    }

    type DeviceElements = Vec<DeviceElementsEnum>;

    #[derive(Deserialize, Debug)]
    enum Parameters {
        parameter,
        RealParameter,
        BoolParameter,
        IntegerParameter,
        EnumParameter,
        TimeSignatureParameter,
    }

    #[derive(Deserialize, Debug)]
    struct Device {
        // Extends referenceable
        #[serde(rename = "@id")]
        id: String,
        #[serde(rename = "$value")]
        #[serde(default)]
        device_elements: DeviceElements,
        #[serde(rename = "@deviceID")]
        device_id: Option<String>,
        #[serde(rename = "@deviceName")]
        device_name: Option<String>,
        #[serde(rename = "@deviceRole")]
        device_role: Option<DeviceRole>,
        #[serde(rename = "@deviceVendor")]
        device_vendor: Option<String>,
        #[serde(rename = "@loaded")]
        loaded: Option<bool>,
    }

    #[derive(Deserialize, Debug)]
    struct Plugin {
        // Extends device
        #[serde(rename = "@id")]
        id: String,
        #[serde(rename = "$value")]
        #[serde(default)]
        device_elements: DeviceElements,
        #[serde(rename = "@deviceID")]
        device_id: Option<String>,
        #[serde(rename = "@deviceName")]
        device_name: Option<String>,
        #[serde(rename = "@deviceRole")]
        device_role: Option<DeviceRole>,
        #[serde(rename = "@deviceVendor")]
        device_vendor: Option<String>,
        #[serde(rename = "@loaded")]
        loaded: Option<bool>,
        // End of extension
        #[serde(rename = "@pluginVersion")]
        plugin_version: Option<String>,
    }

    #[derive(Deserialize, Debug)]
    struct Vst2Plugin {
        // Extends plugin
        #[serde(rename = "@id")]
        id: String,
        #[serde(rename = "$value")]
        #[serde(default)]
        device_elements: DeviceElements,
        #[serde(rename = "@deviceID")]
        device_id: Option<String>,
        #[serde(rename = "@deviceName")]
        device_name: Option<String>,
        #[serde(rename = "@deviceRole")]
        device_role: Option<DeviceRole>,
        #[serde(rename = "@deviceVendor")]
        device_vendor: Option<String>,
        #[serde(rename = "@loaded")]
        loaded: Option<bool>,
        // End of extension
        #[serde(rename = "@pluginVersion")]
        plugin_version: Option<String>,
    }

    #[derive(Deserialize, Debug)]
    struct Vst3Plugin {
        // Extends Plugin
        #[serde(rename = "@id")]
        id: String,
        #[serde(rename = "$value")]
        #[serde(default)]
        device_elements: DeviceElements,
        #[serde(rename = "@deviceID")]
        device_id: Option<String>,
        #[serde(rename = "@deviceName")]
        device_name: Option<String>,
        #[serde(rename = "@deviceRole")]
        device_role: Option<DeviceRole>,
        #[serde(rename = "@deviceVendor")]
        device_vendor: Option<String>,
        #[serde(rename = "@loaded")]
        loaded: Option<bool>,
        // End of extension
        #[serde(rename = "@pluginVersion")]
        plugin_version: Option<String>,
    }

    #[derive(Deserialize, Debug)]
    struct ClapPlugin {
        // Extends Plugin
        #[serde(rename = "@id")]
        id: String,
        #[serde(rename = "$value")]
        #[serde(default)]
        device_elements: DeviceElements,
        #[serde(rename = "@deviceID")]
        device_id: Option<String>,
        #[serde(rename = "@deviceName")]
        device_name: Option<String>,
        #[serde(rename = "@deviceRole")]
        device_role: Option<DeviceRole>,
        #[serde(rename = "@deviceVendor")]
        device_vendor: Option<String>,
        #[serde(rename = "@loaded")]
        loaded: Option<bool>,
        // End of extension
        #[serde(rename = "@pluginVersion")]
        plugin_version: Option<String>,
    }

    #[derive(Deserialize, Debug)]
    struct BuiltinDevice {
        // Extends device
        #[serde(rename = "@id")]
        id: String,
        #[serde(rename = "$value")]
        #[serde(default)]
        device_elements: DeviceElements,
        #[serde(rename = "@deviceID")]
        device_id: Option<String>,
        #[serde(rename = "@deviceName")]
        device_name: Option<String>,
        #[serde(rename = "@deviceRole")]
        device_role: Option<DeviceRole>,
        #[serde(rename = "@deviceVendor")]
        device_vendor: Option<String>,
        #[serde(rename = "@loaded")]
        loaded: Option<bool>,
    }

    #[derive(Deserialize, Debug)]
    enum EqBandParamsEnum {
        Freq(RealParameter),
        Gain(RealParameter),
        Q(RealParameter),
        Enabled(BoolParameter),
    }

    #[derive(Deserialize, Debug)]
    struct EqBand {
        #[serde(default)]
        eq_band_params: Vec<EqBandParamsEnum>,
        #[serde(rename = "@type")]
        eq_type: EqBandTypeEnum,
        #[serde(rename = "@order")]
        order: Option<i32>,
    }

    #[derive(Deserialize, Debug)]
    enum EqBandTypeEnum {
        highPass,
        lowPass,
        bandPass,
        highShelf,
        lowShelf,
        bell,
        notch,
    }

    #[derive(Deserialize, Debug)]
    enum EqParamsEnum {
        Band(EqBand),
        InputGain(RealParameter),
        OutputGain(RealParameter),
    }

    #[derive(Deserialize, Debug)]
    struct Equalizer {
        // Extends builtinDevice
        #[serde(rename = "@id")]
        id: String,
        #[serde(rename = "$value")]
        #[serde(default)]
        device_elements: DeviceElements,
        #[serde(rename = "@deviceID")]
        device_id: Option<String>,
        #[serde(rename = "@deviceName")]
        device_name: Option<String>,
        #[serde(rename = "@deviceRole")]
        device_role: Option<DeviceRole>,
        #[serde(rename = "@deviceVendor")]
        device_vendor: Option<String>,
        #[serde(rename = "@loaded")]
        loaded: Option<bool>,
        // End of extension
        #[serde(default)]
        eq_band_params: Vec<EqParamsEnum>,
    }

    use crate::real_parameter::RealParameter;

    #[derive(Deserialize, Debug)]
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

    #[derive(Deserialize, Debug)]
    struct Compressor {
        // Extendes builtInDevice
        #[serde(rename = "@id")]
        id: String,
        #[serde(rename = "$value")]
        #[serde(default)]
        device_elements: DeviceElements,
        #[serde(rename = "@deviceID")]
        device_id: Option<String>,
        #[serde(rename = "@deviceName")]
        device_name: Option<String>,
        #[serde(rename = "@deviceRole")]
        device_role: Option<DeviceRole>,
        #[serde(rename = "@deviceVendor")]
        device_vendor: Option<String>,
        #[serde(rename = "@loaded")]
        loaded: Option<bool>,
        // Extension ends
        #[serde(default)]
        params: CompressorParams,
    }

    #[derive(Deserialize, Debug)]
    enum NoiseGateParamsEnum {
        Attack(RealParameter),
        Range(RealParameter),
        Ratio(RealParameter),
        Release(RealParameter),
        Threshold(RealParameter),
    }

    type NoiseGateParams = Vec<NoiseGateParamsEnum>;

    #[derive(Deserialize, Debug)]
    struct NoiseGate {
        // Extendes builtInDevice
        #[serde(rename = "@id")]
        id: String,
        #[serde(rename = "$value")]
        #[serde(default)]
        device_elements: DeviceElements,
        #[serde(rename = "@deviceID")]
        device_id: Option<String>,
        #[serde(rename = "@deviceName")]
        device_name: Option<String>,
        #[serde(rename = "@deviceRole")]
        device_role: Option<DeviceRole>,
        #[serde(rename = "@deviceVendor")]
        device_vendor: Option<String>,
        #[serde(rename = "@loaded")]
        loaded: Option<bool>,
        // Extension ends
        #[serde(default)]
        params: NoiseGateParams,
    }

    #[derive(Deserialize, Debug)]
    enum LimiterParamsEnum {
        Attack(RealParameter),
        InputGain(RealParameter),
        OutputGain(RealParameter),
        Release(RealParameter),
        Threshold(RealParameter),
    }

    type LimiterParams = Vec<LimiterParamsEnum>;

    #[derive(Deserialize, Debug)]
    struct Limiter {
        // Extendes builtInDevice
        #[serde(rename = "@id")]
        id: String,
        #[serde(rename = "$value")]
        #[serde(default)]
        device_elements: DeviceElements,
        #[serde(rename = "@deviceID")]
        device_id: Option<String>,
        #[serde(rename = "@deviceName")]
        device_name: Option<String>,
        #[serde(rename = "@deviceRole")]
        device_role: Option<DeviceRole>,
        #[serde(rename = "@deviceVendor")]
        device_vendor: Option<String>,
        #[serde(rename = "@loaded")]
        loaded: Option<bool>,
        // Extension ends
        #[serde(default)]
        params: LimiterParams,
    }

    #[derive(Deserialize, Debug)]
    struct AuPlugin {
        // Extends Plugin
        #[serde(rename = "@id")]
        id: String,
        #[serde(rename = "$value")]
        #[serde(default)]
        device_elements: DeviceElements,
        #[serde(rename = "@deviceID")]
        device_id: Option<String>,
        #[serde(rename = "@deviceName")]
        device_name: Option<String>,
        #[serde(rename = "@deviceRole")]
        device_role: Option<DeviceRole>,
        #[serde(rename = "@deviceVendor")]
        device_vendor: Option<String>,
        #[serde(rename = "@loaded")]
        loaded: Option<bool>,
        // End of extension
        #[serde(rename = "@pluginVersion")]
        plugin_version: Option<String>,
    }

    #[derive(Deserialize, Debug)]
    enum DeviceTypes {
        Device(Device),
        Vst2Plugin(Vst2Plugin),
        Vst3Plugin(Vst3Plugin),
        ClapPlugin(ClapPlugin),
        BuiltinDevice(BuiltinDevice),
        Equalizer(Equalizer),
        Compressor(Compressor),
        NoiseGate(NoiseGate),
        Limiter(Limiter),
        AuPlugin(AuPlugin),
    }

    #[derive(Deserialize, Debug)]
    struct Devices {
        #[serde(default)]
        #[serde(rename = "$value")]
        devices: Vec<DeviceTypes>,
    }

    // #[derive(Deserialize)]
    // enum SendParamsEnum {
    //     Pan(RealParameter),
    //     Volume(RealParameter),
    // }

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

    // type SendParams = Vec<SendParamsEnum>;

    #[derive(Deserialize)]
    struct Send {
        // Extends referenceable
        #[serde(rename = "@id")]
        id: String,
        #[serde(rename = "Volume")]
        volume: RealParameter,
        #[serde(rename = "Pan")]
        pan: Option<RealParameter>,
        #[serde(rename = "@destination")]
        destination: Channel,
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
        Pan(RealParameter),
        Mute(BoolParameter),
        Volume(RealParameter),
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
    struct Channel {
        // Extends lane
        #[serde(rename = "@id")]
        id: String,
        #[serde(rename = "@name")]
        name: Option<String>, // attribute
        #[serde(rename = "@color")]
        color: Option<String>, // att
        #[serde(rename = "@comment")]
        comment: Option<String>, // att
        #[serde(rename = "$value")]
        #[serde(default)]
        channel_elements: ChannelElements,
        #[serde(rename = "@audioChannels")]
        audio_channels: i32,
        #[serde(rename = "@destination")]
        destination: Option<String>,
        #[serde(rename = "@role")]
        role: Option<MixerRoleEnum>,
        #[serde(rename = "@solo")]
        solo: Option<bool>,
    }

    let mut obj: Channel = from_str(xml).unwrap();

    println!("Deserialized object {:?} ", obj);
}

#[test]
fn parse_structure() {
    // XS:scheme for structure

    /*
    <xs:element name="Structure" minOccurs="0">
            <xs:complexType>
              <xs:sequence>
                <xs:choice minOccurs="0" maxOccurs="unbounded">
                  <xs:element ref="Track"/>
                  <xs:element ref="Channel"/>
                </xs:choice>
              </xs:sequence>
            </xs:complexType>
          </xs:element>
     */

    let xml = r##"
    <Structure>
    <Track contentType="notes" loaded="true" id="id2" name="Bass" color="#a2eabf">
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
      </Channel>
    </Track>
    <Track contentType="audio" loaded="true" id="id9" name="Drumloop" color="#b53bba">
      <Channel audioChannels="2" destination="id15" role="regular" solo="false" id="id10">
        <Mute value="false" id="id13" name="Mute"/>
        <Pan max="1.000000" min="0.000000" unit="normalized" value="0.500000" id="id12" name="Pan"/>
        <Volume max="2.000000" min="0.000000" unit="linear" value="0.177125" id="id11" name="Volume"/>
      </Channel>
    </Track>
    <Track contentType="audio notes" loaded="true" id="id14" name="Master">
      <Channel audioChannels="2" role="master" solo="false" id="id15">
        <Mute value="false" id="id18" name="Mute"/>
        <Pan max="1.000000" min="0.000000" unit="normalized" value="0.500000" id="id17" name="Pan"/>
        <Volume max="2.000000" min="0.000000" unit="linear" value="1.000000" id="id16" name="Volume"/>
      </Channel>
    </Track>
  </Structure>"##;
    use crate::channel::Channel;
    use crate::track::Track;

    #[derive(Deserialize, Debug)]
    enum TrackChannelEnum {
        Track(Track),
        Channel(Channel),
    }

    #[derive(Deserialize, Debug)]
    struct Structure {
        #[serde(rename = "$value")]
        sequence: Vec<TrackChannelEnum>,
    }

    let mut obj: Structure = from_str(xml).unwrap();

    println!("Deserialized object {:#?} ", obj);
}

#[test]
fn parse_transport() {
    let xml = r##"
    <Transport>
    <Tempo max="666.000000" min="20.000000" unit="bpm" value="149.000000" id="id0" name="Tempo"/>
    <TimeSignature denominator="4" numerator="4" id="id1"/>
  </Transport>
  "##;
    use crate::transport::Transport;

    let mut obj: Transport = from_str(xml).unwrap();

    println!("Deserialized object {:#?} ", obj);
}
