mod application;
mod arrangement;
mod bool_parameter;
mod channel;
mod content_type;
mod daw_project;
mod device;
mod enum_parameter;
mod expression_type;
mod file_reference;
mod integer_parameter;
mod interpolation;
mod lane;
mod meta_data;
mod mixer_role;
mod parameter;
mod project;
mod real_parameter;
mod scene;
mod send;
mod send_type;
mod time_signature_parameter;
mod timeline;
mod track;
mod transport;
mod unit;
mod utility;

use std::sync::atomic::{AtomicI32, Ordering};

static ID_XML: AtomicI32 = AtomicI32::new(0);

fn add_one_get() -> i32 {
    ID_XML.fetch_add(1, Ordering::SeqCst)
}

pub fn reset_xml_id() {
    let _ = ID_XML.fetch_and(0, Ordering::SeqCst);
}

#[cfg(test)]
#[test]
fn load_daw_project_test() {
    use crate::project::Project;
    use quick_xml::de::from_str;

    let xml = r##"
    <?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Project version="1.0">
  <Application name="Bitwig Studio" version="5.0"/>
  <Transport>
    <Tempo max="666.000000" min="20.000000" unit="bpm" value="149.000000" id="id0" name="Tempo"/>
    <TimeSignature denominator="4" numerator="4" id="id1"/>
  </Transport>
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
  </Structure>
  <Arrangement id="id19">
    <Lanes timeUnit="beats" id="id20">
      <Lanes track="id2" id="id21">
        <Clips id="id22">
          <Clip time="0.0" duration="8.0" playStart="0.0">
            <Notes id="id23">
              <Note time="0.000000" duration="0.250000" channel="0" key="65" vel="0.787402" rel="0.787402"/>
              <Note time="1.000000" duration="0.250000" channel="0" key="65" vel="0.787402" rel="0.787402"/>
              <Note time="4.000000" duration="0.250000" channel="0" key="65" vel="0.787402" rel="0.787402"/>
              <Note time="5.000000" duration="0.250000" channel="0" key="65" vel="0.787402" rel="0.787402"/>
              <Note time="0.500000" duration="0.250000" channel="0" key="64" vel="0.787402" rel="0.787402"/>
              <Note time="4.500000" duration="0.250000" channel="0" key="64" vel="0.787402" rel="0.787402"/>
              <Note time="1.500000" duration="2.500000" channel="0" key="53" vel="0.787402" rel="0.787402"/>
              <Note time="5.500000" duration="0.250000" channel="0" key="53" vel="0.787402" rel="0.787402"/>
              <Note time="6.000000" duration="2.000000" channel="0" key="53" vel="0.787402" rel="0.787402"/>
            </Notes>
          </Clip>
        </Clips>
      </Lanes>
      <Lanes track="id9" id="id24">
        <Clips id="id25">
          <Clip time="0.0" duration="8.00003433227539" playStart="0.0" loopStart="0.0" loopEnd="8.00003433227539" fadeTimeUnit="beats" fadeInTime="0.0" fadeOutTime="0.0" name="Drumfunk3 170bpm">
            <Clips id="id26">
              <Clip time="0.0" duration="8.00003433227539" contentTimeUnit="beats" playStart="0.0" fadeTimeUnit="beats" fadeInTime="0.0" fadeOutTime="0.0">
                <Warps contentTimeUnit="seconds" timeUnit="beats" id="id28">
                  <Audio algorithm="stretch" channels="2" duration="2.823541666666667" sampleRate="48000" id="id27">
                    <File path="audio/Drumfunk3 170bpm.wav"/>
                  </Audio>
                  <Warp time="0.0" contentTime="0.0"/>
                  <Warp time="8.00003433227539" contentTime="2.823541666666667"/>
                </Warps>
              </Clip> 
            </Clips>
          </Clip>
        </Clips>
      </Lanes>
      <Lanes track="id14" id="id29">
        <Clips id="id30"/>
      </Lanes>
    </Lanes>
  </Arrangement>
  <Scenes/>
</Project>
    "##;

    let mut obj: Project = from_str(xml).unwrap();

    println!("Deserialized object {:#?}", obj);
}

#[derive(PartialEq)]
pub enum Features {
    CUE_MARKERS,
    CLIPS,
    AUDIO,
    NOTES,
    AUTOMATION,
    ALIAS_CLIPS,
    PLUGINS,
}
mod project_creator {

    use crate::arrangement::Arrangement;
    use crate::channel::{Channel, DeviceTypes};
    use crate::content_type::ContentType;
    use crate::device::device::DeviceElementsEnum;
    use crate::device::device_role::DeviceRole;
    use crate::device::vst3_plugin::Vst3Plugin;
    use crate::file_reference::FileReference;
    use crate::track::TrackChannelEnum;
    use crate::utility::{self, create_track};
    use crate::{arrangement, Features};
    use crate::{project::Project, reset_xml_id};

    pub fn create_empty_project() -> Project {
        reset_xml_id();
        Project::new_test("Test".to_string(), 1.0)
    }

    pub fn create_dummy_project(num_tracks: i32, features: Vec<Features>) -> Project {
        let mut project = create_empty_project();

        let mut master_track = create_track(
            "Master".to_string(),
            vec![],
            crate::mixer_role::MixerRoleEnum::Master,
            1.0,
            0.5,
        );

        if features.contains(&Features::PLUGINS) {
            // create the device and add it to channel devices struct of master track
        }

        // create arrangement
        // add lanes to arrangement

        if features.contains(&FEATURES::CUE_MARKERS) {
            // create markers

            // add some markers to cue markers
        }

        for i in 0..num_tracks {
            let mut track = utility::create_track(
                format!("Track {}", (i + 1).to_string()),
                vec![ContentType::notes],
                crate::mixer_role::MixerRoleEnum::Regular,
                1.0,
                0.5,
            );

            // create tracks add to structure

            // add color

            // add desitnation

            // track lanes to arrangament lanes

            // if there are clips

            // create clips

            // if there are alias clips

            // create them

            // if there is automation

            // add some
        }
        /*
        1. RealPoint and Marker Structs
        Create a RealPoint struct with fields time, value, and interpolation.
        Create a Marker struct with fields time and name.
        2. Enum Definitions
        Define an Interpolation enum.
        Define an AudioScenario enum.
        3. Project Struct
        Create a Project struct.
        Include fields like structure, scenes, arrangement, transport, etc.
        4. RealParameter and Lanes Structs
        Create a RealParameter struct for tempo values.
        Create a Lanes struct with timeUnit field.
        5. Utility Functions
        Implement utility functions to create tracks, clips, audio, and warps.
        Implement functions to save projects.
        6. Implement Tests
        Implement tests similar to Java methods, converting assertions to Rust equivalents.
        Handle file operations using Rust's file handling libraries.
        7. Stream Handling
        Implement stream handling for reading/writing project files.
        Implement methods to load embedded files.
        8. Export Schema
        Implement functions to export schema for MetaData and Project.
        9. Serialization
        Implement serialization/deserialization functions for DoubleAdapter and related tests.
        10. Main Functions
        Implement main functions to create and manipulate projects, tracks, and clips.
        11. Error Handling
        Implement error handling using Rust's Result type for file operations and other potential errors.
        12. Trait Implementations
        Implement necessary traits like Debug, Clone, etc., for custom structs.
         */

        struct RealPoint {
            time: f64,
            value: f64,
            interpolation: Interpolation,
        }

        struct Marker {
            time: f64,
            name: String,
        }

        #[derive(Debug)]
        enum Interpolation {
            Linear,
            Cubic,
        }

        #[derive(Debug)]
        enum AudioScenario {
            Scenario1,
            Scenario2,
        }

        struct Project {
            structure: Vec<RealPoint>,
            scenes: Vec<Scene>,
            arrangement: Arrangement,
            transport: Transport,
            // Other fields...
        }

        struct RealParameter {
            value: f64,
        }

        struct Lanes {
            time_unit: TimeUnit,
        }

        // Functions to create tracks, clips, audio, and warps...

        fn create_track() -> Track {
            // Implementation
        }

        fn create_clip() -> Clip {
            // Implementation
        }

        fn save_project(project: &Project, filename: &str) -> Result<(), Error> {
            // Implementation
            Ok(())
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn test_create_track() {
                // Test creation of a track
            }

            #[test]
            fn test_create_clip() {
                // Test creation of a clip
            }

            #[test]
            fn test_save_project() {
                // Test saving a project
            }

            // Other tests...
        }

        // Implement stream handling for reading/writing project files...

        fn read_project_file(filename: &str) -> Result<Project, Error> {
            // Implementation
        }

        fn write_project_file(project: &Project, filename: &str) -> Result<(), Error> {
            // Implementation
            Ok(())
        }

        // Implement functions to export schema for MetaData and Project...

        fn export_metadata_schema() -> Schema {
            // Implementation
        }

        fn export_project_schema() -> Schema {
            // Implementation
        }

        // Implement serialization/deserialization functions for DoubleAdapter and related tests...

        impl Serialize for DoubleAdapter {
            // Implementation
        }

        impl Deserialize for DoubleAdapter {
            // Implementation
        }

        // Implement main functions to create and manipulate projects, tracks, and clips...

        fn main() {
            // Implementation
        }

        // Implement error handling using Rust's Result type for file operations and other potential errors...

        fn load_project(filename: &str) -> Result<Project, Error> {
            // Implementation
        }

        // Implement necessary traits like Debug, Clone, etc., for custom structs...

        impl Clone for RealPoint {
            fn clone(&self) -> Self {
                RealPoint {
                    time: self.time,
                    value: self.value,
                    interpolation: self.interpolation.clone(),
                }
            }
        }

        impl Debug for Marker {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.debug_struct("Marker")
                    .field("time", &self.time)
                    .field("name", &self.name)
                    .finish()
            }
        }

        // Other trait implementations...

        project
    }
}

#[cfg(test)]
fn save_daw_project_test() {
    // Implementation and tests of serialisation

    use daw_project::DawProject;
}
