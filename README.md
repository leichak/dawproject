# DAWproject Rust Crate

DAWproject
Open exchange format for user data between Digital Audio Workstations (DAWs)

## Motivation
The DAWproject format in Rust provides a vendor-agnostic way to transfer user data between different music applications (DAWs).

Currently, there is no file format purpose-built for this task. Standard MIDI files can represent note data, but often at a lower level than the DAW's internal representation. AAF only covers audio and lacks a concept of musical time. Most plugins allow saving presets, but this must be done for each instance. Users often resort to exporting audio as stems.

The goal of this Rust crate is to export all translatable project data (audio/note/automation/plug-in) along with the structure into a single DAWproject file.

| Feature      | DAWproject        | Standard MIDI Files | Advanced Authoring Format (AAF) |
|--------------|-------------------|----------------------|----------------------------------|
| Intended Use | Music Production | MIDI Sequencing      | Video Post-Production            |
| Time Format  | Beats and seconds can be combined | Beats | Seconds                          |
| Audio        | Audio Events/Clips, Fades, Crossfades, Amplitude, Pan, Time Warping, Transpose | - | Audio Events/Clips, Fades, Crossfades, Amplitude, Pan |
| Notes        | Notes, Note Expressions | Notes | -                                  |
| Automation   | Tempo, Time Signature, MIDI Messages, Volume, Pan, Mute, Sends, Plug-in Parameters, Built-in Device Parameters | Tempo, Time Signature, MIDI Messages, SysEx Messages | Volume, Pan, Video Related Parameters, Plug-ins |

## Status
The format is version 1.0 and stable.

## Goals
- Package all user data of a project/song into a single file.
- Include audio timeline data, note timeline data, note expression data, automation timeline data, and embedded/referenced audio data.
- Preserve as much user-created data as feasible.
- Express the track and timeline structures of the exporting DAW, leaving flattening to the importer.
- Simple to implement.
- Built upon established open standards.
- Language agnostic, no special dependencies.
- Open & free.

## Non-goals
- Being the native file format for a DAW.
- Optimal performance.
- Storing low-level MIDI events directly.
- Storing non-session data (view settings, preferences).

## Format Specification
- File Extension: .dawproject
- Container: ZIP
- Format: XML (project.xml, metadata.xml)
- Text encoding: UTF-8
- The exporting DAW can choose the directory structure for media and plug-in files.

## DAWproject Rust Crate Reference
- The crate is built in Rust.
- It uses plain XML/ZIP and can be used with any programming language that can parse those.
- The DOM is defined by Rust structs with RustDoc comments for documentation.

## Building the Library, Documentation, and Tests
Requires Rust to build.

To build:

```bash
cargo build
```

## Example Project
The exporting application is free to structure tracks and timelines. The choice is left to the importing application to either use the provided structure or flatten/convert it.

As an example, here's the project.xml of a simple file saved in Bitwig Studio 5.0 with one instrument track and one audio track.

```xml
<!-- Example XML content goes here -->
```

## DAW Support
DAWproject 1.0 is currently supported by the following DAWs:

- Bitwig Studio 5.0.9
- PreSonus Studio One 6.5

Feel free to contribute to adding support for more DAWs.

## To-Do List
- [x] Reflect all structures in files and appropriate mods.
- [ ] Implement XML Serialization using quick_xml and serde.
- - [x] Move to quick-xml 
- - [x] Employ Serde deserialization using Rust types
- - - [x] Derive Deserialize for all Project related types
- - - - [x] Deserialize Track
- - - - [x] Deserialize Channel
- - - - [x] Deserialize Structure
- - - - [x] Deserialize Application
- - - - [x] Deserialize Transport
- - - - [x] Deserialize Arrangement
- - - - [x] Deserialize Scenes
- - - - [x] Deserialize Full Project
- - - - [x] Test Deserializing code
- - - - [x] Review and clean-up
- - [ ] Derive Serialize for all Project related types
- - - - [x] Serialize Track
- - - - [x] Serialize Channel
- - - - [x] Serialize Structure
- - - - [x] Serialize Application
- - - - [x] Serialize Transport
- - - - [x] Serialize Arrangement
- - - - [x] Serialize Scenes
- - - - [x] Serialize Full Project
- - - - [in_progress] Test Serializing code
- - - - [x] write DawProject Struct
- - - - [x] continue lib.rs
- - - - [ ] handle xml id
- - - - [ ] Review and clean-up
- [ ] Create main file loading mod 
= [ ] Write tests 



Ideas:
Maybe write tests for the Java compatibility