/*
This struct represents project, it conists following data:
- version of DawProject format his file was saved on
- metadata (name/version) about the application that saved this file
- transport element containing playback parameters such as Tempo and Time-signatures
- track/channel structure of this file (list of Lane)
- arrangement timeline of this file
- Clip Launcher scenes of this file
 */

use crate::application::Application;
use crate::arrangement::Arrangement;
use crate::lane::Lane;
use crate::scene::Scene;
use crate::transport::Transport;
struct Project {
    pub version: String,
    application: Application,
    transport: Transport,
    line: Vec<Lane>,
    arrangement: Arrangement,
    scenes: Vec<Scene>,
}
