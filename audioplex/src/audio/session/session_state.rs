use crate::error::Error;
use windows::Win32::Media::Audio::{
    AudioSessionState, AudioSessionStateActive, AudioSessionStateExpired, AudioSessionStateInactive,
};

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

impl Into<AudioSessionState> for SessionState {
    fn into(self) -> AudioSessionState {
        match self {
            Self::Inactive => AudioSessionStateInactive,
            Self::Active => AudioSessionStateActive,
            Self::Expired => AudioSessionStateExpired,
        }
    }
}
