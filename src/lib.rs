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

    use std::collections::HashMap;
    use std::fmt::format;
    use std::fs::File;
    use std::path::Path;
    use std::sync::Arc;

    use crate::arrangement::{Arrangement, ArrangementSequenceEnum};
    use crate::channel::{Channel, ChannelElementsEnum, DeviceTypes, Devices};
    use crate::content_type::ContentType;
    use crate::daw_project::DawProject;
    use crate::device::device::DeviceElementsEnum;
    use crate::device::device_role::DeviceRole;
    use crate::device::vst3_plugin::Vst3Plugin;
    use crate::file_reference::FileReference;
    use crate::interpolation::{Interpolation, InterpolationEnum};
    use crate::meta_data::MetaData;
    use crate::project::TrackChannelEnum;
    use crate::real_parameter::RealParameter;
    use crate::timeline::UpcastTimeline;
    use crate::timeline::audio::{self, Audio};
    use crate::timeline::automation_target::AutomationTarget;
    use crate::timeline::clip::{self, Clip};
    use crate::timeline::clips::Clips;
    use crate::timeline::lanes::{ArrangementTypeChoiceEnum, Lanes};
    use crate::timeline::marker::Marker;
    use crate::timeline::markers::Markers;
    use crate::timeline::note::Note;
    use crate::timeline::notes::Notes;
    use crate::timeline::point::Point;
    use crate::timeline::points::{Points, PointsSequenceEnum, PointsTypeEnum};
    use crate::timeline::real_point::RealPoint;
    use crate::timeline::time_unit::TimeUnit;
    use crate::timeline::timeline::TimeLine;
    use crate::timeline::warps::{Warps, WarpsSequenceEnum};
    use crate::track::Track;
    use crate::transport::{Transport, TransportSequence};
    use crate::unit::Unit;
    use crate::utility::{self, create_track};
    use crate::{arrangement, Features};
    use crate::{project::Project, reset_xml_id};
    use strum::{EnumIter, IntoEnumIterator, VariantNames};

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

        let mut master_track_ref = &mut master_track;

        if features.contains(&Features::PLUGINS) {
            let mut device = Vst3Plugin::new_empty();
            device.device_role = Some(DeviceRole::audioFX);
            if let Some(state) = device.device_elements.iter_mut().find(|el| match el {
                DeviceElementsEnum::State(_) => true,
                _ => false,
            }) {
                match state {
                    DeviceElementsEnum::State(state) => {
                        state.path = "plugin-states/12323545.vstpreset".to_string();
                    }
                    _ => (),
                }
            } else {
                device
                    .device_elements
                    .push(DeviceElementsEnum::State(FileReference::new(
                        "plugin-states/12323545.vstpreset".to_string(),
                    )));
            }

            if let Some(c) = master_track_ref
                .track_channel
                .iter_mut()
                .find(|el| match el {
                    TrackChannelEnum::Channel(_) => true,
                    _ => false,
                })
            {
                match c {
                    TrackChannelEnum::Channel(c) => {
                        if let Some(devices) = c.channel_elements.iter_mut().find(|el| match el {
                            ChannelElementsEnum::Devices(_) => true,
                            _ => false,
                        }) {
                            match devices {
                                ChannelElementsEnum::Devices(devices) => {
                                    devices.devices.push(DeviceTypes::Vst3Plugin(device))
                                }
                                _ => (),
                            }
                        } else {
                            c.channel_elements
                                .push(ChannelElementsEnum::Devices(Devices {
                                    devices: vec![DeviceTypes::Vst3Plugin(device)],
                                }));
                        }
                    }
                    _ => (),
                }
            } else {
                master_track_ref
                    .track_channel
                    .push(TrackChannelEnum::Channel(Channel::new_test(
                        1.0,
                        0.0,
                        crate::mixer_role::MixerRoleEnum::Master,
                    )))
            }
        }

        project
            .structure
            .as_mut()
            .unwrap()
            .sequence
            .push(TrackChannelEnum::Track(master_track));

        let mut arrangement = Arrangement::new_test();
        if arrangement.sequence.is_none() {
            arrangement.sequence = Some(vec![]);
        }
        let mut arrangement_lanes = Lanes::new_empty();

        arrangement_lanes.time_unit = Some(TimeUnit::beats);

        if features.contains(&Features::CUE_MARKERS) {
            let mut cue_markers = Markers::new_empty();
            cue_markers
                .markers
                .as_mut()
                .unwrap()
                .push(Marker::new(0.0, "Verse".to_string()));
            cue_markers
                .markers
                .as_mut()
                .unwrap()
                .push(Marker::new(24.0, "Chorus".to_string()));
        }

        for i in 0..num_tracks {
            let mut track = utility::create_track(
                format!("Track {}", (i + 1).to_string()),
                vec![ContentType::notes],
                crate::mixer_role::MixerRoleEnum::Regular,
                1.0,
                0.5,
            );
            track.color = Some(format!("#{}{}{}{}{}{}", i, i, i, i, i, i).to_string());

            if let Some(c) = track.track_channel.iter_mut().find(|el| match el {
                TrackChannelEnum::Channel(_) => true,
                _ => false,
            }) {
                match c {
                    TrackChannelEnum::Channel(c) => {
                        if let Some(r) = project.get_master_track() {
                            c.destination = Some(r.get_id());
                        }
                    }
                    _ => (),
                }
            }

            let mut track_lanes = Lanes::new_empty(); // move it later to project

            if features.contains(&Features::CLIPS) {
                let mut clips = Clips::new_empty();

                let mut clip = Clip::new_empty();
                clip.name = Some(format!("Clip {}", i));
                clip.time = 8.0 * i as f64;
                clip.duration = Some(4.0);

                let mut notes = Notes::new_empty();
                // add to clip

                for j in 0..8 {
                    let mut note = Note::new_empty();
                    note.key = Some(36 + 12 * (j % (1 + i)));
                    note.vel = Some(0.8);
                    note.rel = Some(0.5);
                    note.time = Some(0.5 * (j as f64));
                    note.duration = Some(0.5);
                    notes.notes_sequence.as_mut().unwrap().push(note);
                }

                if i == 0 && features.contains(&Features::AUTOMATION) {
                    let mut points = Points::new_empty();
                    //  points.target.parameter = track.channel.volume;
                    let mut automation_target = AutomationTarget::new_empty();
                    automation_target.parameter = track
                        .track_channel
                        .iter()
                        .find(|el| match el {
                            TrackChannelEnum::Channel(_) => true,
                            _ => false,
                        })
                        .map(|el| match el {
                            TrackChannelEnum::Channel(c) => {
                                c.volume.as_ref().unwrap().id.as_ref().unwrap().clone()
                            }
                            _ => "".to_string(),
                        });
                    points
                        .points
                        .as_mut()
                        .unwrap()
                        .push(PointsSequenceEnum::Target(automation_target));

                    if points.points.is_none() {
                        points.points = Some(vec![]);
                    }
                    let mut point = PointsSequenceEnum::PointType(PointsTypeEnum::RealPoint(
                        create_point(0.0, 0.0, InterpolationEnum::Linear),
                    ));
                    let mut point_1 = PointsSequenceEnum::PointType(PointsTypeEnum::RealPoint(
                        create_point(8.0, 1.0, InterpolationEnum::Linear),
                    ));

                    points
                        .points
                        .as_mut()
                        .unwrap()
                        .append(&mut vec![point, point_1]);

                    track_lanes
                        .lanes_sequence
                        .as_mut()
                        .unwrap()
                        .push(ArrangementTypeChoiceEnum::Points(points));
                }
                // add clip to clips
                clips.clips.as_mut().unwrap().push(clip);

                if features.contains(&Features::ALIAS_CLIPS) {
                    let mut clip_2 = Clip::new_empty();
                    clip_2.name = Some(format!("Alias Clip {}", i));
                    clip_2.time = 32.0 + 8.0 * i as f64;
                    clip_2.duration = Some(4.0);
                    clip_2.reference = notes.id.clone();
                    clips.clips.as_mut().unwrap().push(clip_2);
                }

                // add clips to track_lanes
                let clips = ArrangementTypeChoiceEnum::Clips(clips);
                track_lanes.lanes_sequence.as_mut().unwrap().push(clips);
                track_lanes.track = Some(track.get_id());
            }

            arrangement_lanes
                .lanes_sequence
                .as_mut()
                .unwrap()
                .push(ArrangementTypeChoiceEnum::Lanes(track_lanes));
        }

        arrangement
            .sequence
            .as_mut()
            .unwrap()
            .push(ArrangementSequenceEnum::Lanes(arrangement_lanes));

        project.arrangement = Some(arrangement);

        project
    }

    fn create_point(time: f64, value: f64, interpolation: InterpolationEnum) -> RealPoint {
        RealPoint {
            time: Some(time),
            value: Some(value),
            interpolation: Some(Interpolation::create(interpolation)),
        }
    }

    pub fn create_marker(time: f64, name: String) -> Marker {
        Marker::new(time, name)
    }

    #[test]
    pub fn save_daw_project() -> Result<(), ()> {
        let mut features = vec![Features::CLIPS, Features::NOTES, Features::AUDIO];
        let proj = create_dummy_project(3, features);
        let meta_data = MetaData::new();
        let mut embedded_files: HashMap<&Path, String> = HashMap::new();

        DawProject::save(
            proj,
            meta_data,
            embedded_files,
            Path::new("target/test.dawproject"),
        )?;

        Ok(())
    }

    #[test]
    pub fn validate_daw_project() -> Result<(), ()> {
        let mut features = vec![Features::CLIPS, Features::NOTES, Features::AUDIO];
        let proj = create_dummy_project(3, features);

        Ok(())
    }

    #[test]
    pub fn validate_complex_project() -> Result<(), ()> {
        let mut features = vec![Features::CLIPS, Features::NOTES, Features::AUDIO]; // more features
        let proj = create_dummy_project(3, features);

        Ok(())
    }

    #[test]
    pub fn save_and_load_daw_project() -> Result<(), ()> {
        let mut features = vec![Features::CLIPS, Features::NOTES, Features::AUDIO];
        let proj = create_dummy_project(3, features);
        let meta_data = MetaData::new();
        let mut embedded_files: HashMap<&Path, String> = HashMap::new();
        let file = Path::new("target/test.dawproject");

        DawProject::save(proj, meta_data, embedded_files, file)?;

        let loaded_project = DawProject::load_project(file)?;

        //assert_eq!(proj.structure.unwrap(), loaded_project.structure.unwrap());
        //assert_eq!(proj.scenes.unwrap(), loaded_project.scenes.unwrap());
        Ok(())
    }

    #[test]
    pub fn save_complex_daw_project() -> Result<(), ()> {
        Ok(())
    }

    #[test]
    pub fn save_and_load_complex_daw_project() -> Result<(), ()> {
        Ok(())
    }

    #[test]
    pub fn write_metadata_schema() -> Result<(), ()> {
        Ok(())
    }

    #[test]
    pub fn write_project_schema() -> Result<(), ()> {
        Ok(())
    }

    #[test]
    pub fn load_embedded_file() -> Result<(), ()> {
        Ok(())
    }

    #[derive(EnumIter, VariantNames, Debug, PartialEq)]
    pub enum AudioScenario {
        Warped,
        RawBeats,
        RawSeconds,
        FileWithAbsolutePath,
        FileWithRelativePath,
    }

    fn should_test_offset_and_fades(scenario: AudioScenario) -> bool {
        match scenario {
            AudioScenario::FileWithAbsolutePath => false,
            AudioScenario::FileWithRelativePath => false,
            _ => true,
        }
    }

    #[test]
    pub fn create_audio_example() -> Result<(), ()> {
        for (scenario, name) in AudioScenario::iter().zip(AudioScenario::VARIANTS.iter()) {
            // create_audio_example(0, 0, scenario, name.to_string(), false);
            // createAudioExample(0, 0, scenario, false);
            // if (shouldTestOffsetAndFades(scenario)) {
            //     createAudioExample(0, 0, scenario, true);
            //     createAudioExample(1, 0, scenario, false);
            //     createAudioExample(0, 1, scenario, false);
            // }
        }

        Ok(())
    }

    pub fn create_project_audio_example(
        play_start_offset: f64,
        clip_time: f64,
        scenario: AudioScenario,
        scenario_name: String,
        with_fades: bool,
    ) -> Result<(), ()> {
        let mut name = format!("Audio {}", scenario_name);
        if with_fades {
            name += &"WithFades";
        }
        if play_start_offset != 0.0 {
            name += &"Offset";
        }
        if clip_time != 0.0 {
            name += &"Clipstart";
        }

        let mut project = create_empty_project();
        let mut master_track = create_track(
            "Master".to_string(),
            vec![],
            crate::mixer_role::MixerRoleEnum::Master,
            1.0,
            0.5,
        );
        let mut audio_track = create_track(
            "Audio".to_string(),
            vec![ContentType::audio],
            crate::mixer_role::MixerRoleEnum::Regular,
            1.0,
            0.5,
        );

        //

        if let Some(c) = audio_track.track_channel.iter_mut().find(|el| match el {
            TrackChannelEnum::Channel(_) => true,
            _ => false,
        }) {
            match c {
                TrackChannelEnum::Channel(c) => c.destination = Some(master_track.get_id()),
                _ => (),
            }
        }

        let mut arrangement = Arrangement::new_test(); // add to proj later
        let mut transport = Transport::new_test();
        let mut tempo = RealParameter::new_test(Unit::Bpm);
        tempo.value = Some(155.0);
        transport.sequence.push(TransportSequence::Tempo(tempo));
        if arrangement.sequence.is_none() {
            arrangement.sequence = Some(vec![]);
        }
        let mut arrangement_lanes = Lanes::new_empty();
        let mut arrangement_in_seconds = scenario == AudioScenario::RawBeats;

        arrangement_lanes.time_unit = if arrangement_in_seconds {
            Some(TimeUnit::seconds)
        } else {
            Some(TimeUnit::beats)
        };

        let mut sample = "white-glasses.wav".to_string();
        let mut audio_clip = Clip::new_empty();
        let mut sample_duration = 3.097;
        let mut audio = utility::create_audio(sample, 44100, 2, sample_duration);

        if scenario == AudioScenario::FileWithAbsolutePath {
            if audio.files_sequence.is_none() {
                audio.files_sequence = Some(vec![]);
            }
            // Create file here and return absolute path 
            let path = utility::create_file_path_absolute_string(format!("test-data/{}", sample)).unwrap();
            audio.files_sequence.as_mut().unwrap().push(FileReference { path: path, external: Some(true) });

        } else if scenario == AudioScenario::FileWithRelativePath {
            if audio.files_sequence.is_none() {
                audio.files_sequence = Some(vec![]);
            }
            audio.files_sequence.as_mut().unwrap().push(FileReference { path: format!("test-data/{}", sample), external: Some(true) });
        }  

        if scenario == AudioScenario::Warped {
            let mut warps = Warps::new_test(TimeUnit::beats);
            warps.warps_sequence.as_mut().unwrap().push(WarpsSequenceEnum::Warp(utility::create_warp(0.0, 0.0)));
            warps.warps_sequence.as_mut().unwrap().push(WarpsSequenceEnum::Warp(utility::create_warp(8.0, sample_duration)));
            audio_clip = utility::create_clip(warps.upcast(), clip_time, 8.0); // I think it is just upcasting to parent class
            audio_clip.content_time_unit = Some(TimeUnit::beats);
            audio_clip.play_start = Some(play_start_offset);
            if with_fades {
                audio_clip.fade_time_unit = Some(TimeUnit::beats);
                audio_clip.fade_in_time = Some(0.25);
                audio_clip.fade_out_time = Some(0.25);
            }                
        } else {
            audio_clip = utility::create_clip(audio.upcast(), clip_time, 8.0); // I think it is just upcasting to parent class
            audio_clip.content_time_unit = Some(TimeUnit::beats);
            audio_clip.play_start = Some(play_start_offset);
            if with_fades {
                audio_clip.fade_time_unit = Some(TimeUnit::beats);
                audio_clip.fade_in_time = Some(0.25);
                audio_clip.fade_out_time = Some(0.25);
            }                
        }

        //let mut clips = utility::create_clips(clips);
        // clips. = audio_track
        // arrangement_lanes . push (clips)

        save_test_project

        Ok(())
    }
}
