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
mod interpolation;
mod lane;
mod meta_data;
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
mod utility;

pub use serde::{Deserialize, Serialize};
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

mod daw_project_test {

    use core::num;

    use crate::arrangement::Arrangement;
    use crate::channel::{Channel, DeviceTypes};
    use crate::content_type::ContentType;
    use crate::device::device_role::DeviceRole;
    use crate::device::vst3_plugin::Vst3Plugin;
    use crate::file_reference::FileReference;
    use crate::mixer_role::MixerRoleEnum;
    use crate::project::Project;
    use crate::timeline::clip::Clip;
    use crate::timeline::clips::Clips;
    use crate::timeline::note::Note;
    use crate::timeline::notes::Notes;
    use crate::timeline::{lanes::Lanes, markers::Markers};
    use crate::track::{Track, TrackChannelEnum};
    use uuid::Uuid;

    fn random_uuid() -> String {}

    #[derive(PartialEq)]
    enum Features {
        CUE_MARKERS,
        CLIPS,
        AUDIO,
        NOTES,
        AUTOMATION,
        ALIAS_CLIPS,
        PLUGINS,
    }
    fn create_empty_project() -> Project {
        let mut project = Project::new();

        project.application.name = "Test".to_string();
        project.application.version = "1.0".to_string();
        project
    }

    fn create_dummy_project(num_tracks: i32, features: Vec<Features>) {
        let mut project = create_empty_project();
        let volume = 1.0;
        let pan = 0.5;
        let mixer_role = Some(MixerRoleEnum::Master);

        let mut master_track =
            Track::new_dummy("Master".to_string(), Vec::new(), mixer_role, volume, pan);

        if features
            .iter()
            .find(|x| (**x) == Features::PLUGINS)
            .is_some()
        {
            let file_ref = FileReference {
                path: "plugin-states/12323545.vstpreset".to_string(),
                external: None,
            };
            // If plugins finds add some
            let device = Vst3Plugin {
                id: Some(Uuid::new_v4().to_string()),
                device_elements: todo!(),
                device_id: None,
                device_name: Some("Limiter".to_string()),
                device_role: Some(DeviceRole::audioFX),
                device_vendor: todo!(),
                loaded: todo!(),
                plugin_version: todo!(),
            };

            for tr_ch in &mut master_track.track_channel {
                match tr_ch {
                    TrackChannelEnum::Channel(channel) => {
                        for el in &mut channel.channel_elements {
                            match el {
                                crate::channel::ChannelElementsEnum::Devices(devices) => {
                                    devices.devices.push(DeviceTypes::Vst3Plugin(device));
                                    break;
                                }
                                crate::channel::ChannelElementsEnum::Pan(_) => (),
                                crate::channel::ChannelElementsEnum::Mute(_) => (),
                                crate::channel::ChannelElementsEnum::Volume(_) => (),
                                crate::channel::ChannelElementsEnum::Sends(_) => (),
                            }
                            break;
                        }
                    }
                    TrackChannelEnum::Track(_) => {}
                }
            }
        }

        let mut arragnement = Arrangement {
            id: todo!(),
            name: todo!(),
            color: todo!(),
            comment: todo!(),
            sequence: todo!(),
        };

        let mut arrangement_lanes = Lanes {
            id: todo!(),
            name: todo!(),
            color: todo!(),
            comment: todo!(),
            track: todo!(),
            timeUnit: todo!(),
            lanes_sequence: todo!(),
        };

        if features
            .iter()
            .find(|x| (**x) == Features::CUE_MARKERS)
            .is_some()
        {
            let mut markers = Markers {
                id: todo!(),
                name: todo!(),
                color: todo!(),
                comment: todo!(),
                track: todo!(),
                timeUnit: todo!(),
                markers: todo!(),
            };
        }

        for i in 0..num_tracks {
            let mut track = Track::new_dummy(
                format!("Track {}", i),
                vec![ContentType::notes],
                Some(MixerRoleEnum::Regular),
                1.0,
                0.5,
            );

            let mut track_lanes = Lanes {
                id: todo!(),
                name: todo!(),
                color: todo!(),
                comment: todo!(),
                track: todo!(),
                timeUnit: todo!(),
                lanes_sequence: todo!(),
            };

            if features.iter().find(|x| (**x) == Features::CLIPS).is_some() {
                let mut clips = Clips {
                    id: todo!(),
                    name: todo!(),
                    color: todo!(),
                    comment: todo!(),
                    track: todo!(),
                    time_unit: todo!(),
                    clips: todo!(),
                };

                let mut clip = Clip {
                    name: todo!(),
                    color: todo!(),
                    comment: todo!(),
                    notes_sequence_choice: todo!(),
                    time: todo!(),
                    duration: todo!(),
                    content_time_unit: todo!(),
                    play_start: todo!(),
                    play_stop: todo!(),
                    loop_start: todo!(),
                    loop_end: todo!(),
                    fade_time_unit: todo!(),
                    fade_in_time: todo!(),
                    fade_out_time: todo!(),
                    reference: todo!(),
                };

                let mut notes = Notes {
                    id: todo!(),
                    name: todo!(),
                    color: todo!(),
                    comment: todo!(),
                    track: todo!(),
                    timeUnit: todo!(),
                    notes_sequence: todo!(),
                };

                for j in 0..8 {
                    let mut note = Note {
                        notes_sequence_choice: todo!(),
                        time: todo!(),
                        duration: todo!(),
                        channel: todo!(),
                        key: todo!(),
                        vel: todo!(),
                        rel: todo!(),
                    };
                }

                if features
                    .iter()
                    .find(|x| (**x) == Features::ALIAS_CLIPS)
                    .is_some()
                {}

                if i == 0
                    && features
                        .iter()
                        .find(|x| (**x) == Features::AUTOMATION)
                        .is_some()
                {}
            }
        }
    }

    fn create_point() {}

    fn create_marker() {}

    fn save_daw_project() {}

    fn validate_daw_project() {}

    fn validate_complex_daw_project() {}

    fn save_and_load_daw_project() {}

    fn save_complex_daw_project() {}

    fn save_and_load_complex_daw_project() {}

    fn write_meta_data_schema() {}

    fn write_project_schema() {}

    fn load_embedded_file() {}

    enum AudioScenario {
        Warped,
        RawBeats,
        RawSeconds,
        FileWithAbsolutePath,
        FileWithRelativePath,
    }

    fn should_test_offset_and_fades() -> bool {
        false
    }

    fn create_audio_example_test() {}

    fn create_audio_example() {}

    fn create_midi_automation_in_clips_example_test() {}

    fn create_midi_automation_examples() {}

    fn double_adapter_test() {}

    fn save_test_project() {}
}
