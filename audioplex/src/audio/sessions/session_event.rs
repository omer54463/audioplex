use crate::audio::sessions::session_state::SessionState;

pub(crate) enum SessionEvent {
    Remove,
    NameChange { display_name: String },
    IconChange { icon_path: String },
    StateChange { session_state: SessionState },
    VolumeChange { volume: f32, mute: bool },
}
