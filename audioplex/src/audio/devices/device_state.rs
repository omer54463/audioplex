use crate::error::Error;
use windows::Win32::Media::Audio::{
    DEVICE_STATEMASK_ALL, DEVICE_STATE_ACTIVE, DEVICE_STATE_DISABLED, DEVICE_STATE_NOTPRESENT,
    DEVICE_STATE_UNPLUGGED,
};

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

impl From<DeviceState> for u32 {
    fn from(value: DeviceState) -> Self {
        match value {
            DeviceState::All => DEVICE_STATEMASK_ALL,
            DeviceState::Active => DEVICE_STATE_ACTIVE,
            DeviceState::Disabled => DEVICE_STATE_DISABLED,
            DeviceState::NotPresent => DEVICE_STATE_NOTPRESENT,
            DeviceState::Unplugged => DEVICE_STATE_UNPLUGGED,
        }
    }
}
