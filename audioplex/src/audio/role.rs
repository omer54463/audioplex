use crate::error::Error;
use windows::Win32::Media::Audio::{eCommunications, eConsole, eMultimedia, ERole};

pub(crate) enum Role {
    Console,
    Communications,
    Multimedia,
}

impl TryFrom<ERole> for Role {
    type Error = Error;

    fn try_from(role: ERole) -> Result<Self, Self::Error> {
        match role {
            role if role == eConsole => Ok(Self::Console),
            role if role == eCommunications => Ok(Self::Communications),
            role if role == eMultimedia => Ok(Self::Multimedia),
            _ => Err(Error::UnknownRole { role }),
        }
    }
}

impl Into<ERole> for Role {
    fn into(self) -> ERole {
        match self {
            Self::Console => eConsole,
            Self::Communications => eCommunications,
            Self::Multimedia => eMultimedia,
        }
    }
}
