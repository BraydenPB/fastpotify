//! No-op tray for platforms without a status-notifier host.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TrayCommand {
    ShowHide,
    PlayPause,
    Next,
    Previous,
    Quit,
}

pub struct TrayService;

impl TrayService {
    pub fn spawn(_wake: impl Fn() + Send + Sync + 'static) -> Option<Self> {
        None
    }

    pub fn drain_commands(&self) -> Vec<TrayCommand> {
        Vec::new()
    }

    pub fn set_playing(&mut self, _playing: bool) {}
}
