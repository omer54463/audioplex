use crate::com::interface_wrapper::InterfaceWrapper;

use super::{session::Session, session_state::SessionState};

pub(crate) enum SessionEvent<'a> {
    Add {
        session_id: String,
        session: InterfaceWrapper<'a, Session>,
    },
    Remove {
        session_id: String,
    },
    NameChange {
        session_id: String,
        display_name: String,
    },
    IconChange {
        session_id: String,
        icon_path: String,
    },
    StateChange {
        session_id: String,
        session_state: SessionState,
    },
    VolumeChange {
        session_id: String,
        volume: f32,
        mute: bool,
    },
}
