//! No-op desktop media controls for platforms without MPRIS.

use crate::player::{Playback, RepeatMode};

#[derive(Clone, Debug, PartialEq)]
pub enum MprisCommand {
    Play,
    Pause,
    PlayPause,
    Stop,
    Next,
    Previous,
    SeekBy(i64),
    SetPosition { track_uri: String, position_ms: u32 },
    SetVolume(f64),
    SetShuffle(bool),
    SetRepeat(RepeatMode),
    OpenUri(String),
    Raise,
    Quit,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MprisTrack {
    pub uri: String,
    pub title: String,
    pub artists: Vec<String>,
    pub album: String,
    pub art_url: Option<String>,
    pub duration_ms: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MprisState {
    pub playback: Playback,
    pub track: Option<MprisTrack>,
    pub position_ms: u32,
    pub volume: f64,
    pub shuffle: bool,
    pub repeat: RepeatMode,
    pub can_control: bool,
}

impl Default for MprisState {
    fn default() -> Self {
        Self {
            playback: Playback::Stopped,
            track: None,
            position_ms: 0,
            volume: 1.0,
            shuffle: false,
            repeat: RepeatMode::Off,
            can_control: true,
        }
    }
}

pub struct MprisService;

impl MprisService {
    pub fn spawn(_wake: impl Fn() + Send + Sync + 'static) -> Self {
        Self
    }

    pub fn drain_commands(&self) -> Vec<MprisCommand> {
        Vec::new()
    }

    pub fn update(&mut self, _state: MprisState) {}

    pub fn seeked(&self, _position_ms: u32) {}
}
