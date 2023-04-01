use windows::Win32::Media::Audio::{
    DEVICE_STATEMASK_ALL, DEVICE_STATE_ACTIVE, DEVICE_STATE_DISABLED, DEVICE_STATE_NOTPRESENT,
    DEVICE_STATE_UNPLUGGED,
};

use crate::error::Error;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum DeviceState {
    All,
    Active,
    Disabled,
    NotPresent,
    Unplugged,
}

impl TryFrom<u32> for DeviceState {
    type Error = Error;

    fn try_from(device_state: u32) -> Result<Self, Self::Error> {
        match device_state {
            DEVICE_STATEMASK_ALL => Ok(Self::All),
            DEVICE_STATE_ACTIVE => Ok(Self::Active),
            DEVICE_STATE_DISABLED => Ok(Self::Disabled),
            DEVICE_STATE_NOTPRESENT => Ok(Self::NotPresent),
            DEVICE_STATE_UNPLUGGED => Ok(Self::Unplugged),
            _ => Err(Error::UnknownDeviceState { device_state }),
        }
    }
}

impl Into<u32> for DeviceState {
    fn into(self) -> u32 {
        match self {
            Self::All => DEVICE_STATEMASK_ALL,
            Self::Active => DEVICE_STATE_ACTIVE,
            Self::Disabled => DEVICE_STATE_DISABLED,
            Self::NotPresent => DEVICE_STATE_NOTPRESENT,
            Self::Unplugged => DEVICE_STATE_UNPLUGGED,
        }
    }
}
