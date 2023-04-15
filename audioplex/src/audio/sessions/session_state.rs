use windows::Win32::Media::Audio::{
    AudioSessionState, AudioSessionStateActive, AudioSessionStateExpired, AudioSessionStateInactive,
};

use crate::error::Error;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum SessionState {
    Inactive,
    Active,
    Expired,
}

impl TryFrom<AudioSessionState> for SessionState {
    type Error = Error;

    fn try_from(session_state: AudioSessionState) -> Result<Self, Self::Error> {
        match session_state {
            AudioSessionStateInactive => Ok(Self::Inactive),
            AudioSessionStateActive => Ok(Self::Active),
            AudioSessionStateExpired => Ok(Self::Expired),
            _ => Err(Error::UnknownSessionState { session_state }),
        }
    }
}

impl From<SessionState> for AudioSessionState {
    fn from(val: SessionState) -> Self {
        match val {
            SessionState::Inactive => AudioSessionStateInactive,
            SessionState::Active => AudioSessionStateActive,
            SessionState::Expired => AudioSessionStateExpired,
        }
    }
}
