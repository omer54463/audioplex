use crate::audio::property_type::PropertyType;
use std::string::FromUtf16Error;
use thiserror::Error;
use windows::Win32::{
    Media::Audio::{EDataFlow, ERole},
    System::Com::{COINIT, STGM},
    UI::Shell::PropertiesSystem::PROPERTYKEY,
};

#[derive(Error, Debug)]
pub(crate) enum Error {
    #[error("Windows error")]
    WindowsError(#[from] ::windows::core::Error),
    #[error("FromUtf16Error error")]
    FromUtf16Error(#[from] FromUtf16Error),
    #[error("Unknown data flow {data_flow:?}")]
    UnknownDataFlow { data_flow: EDataFlow },
    #[error("Unknown device state {device_state:?}")]
    UnknownDeviceState { device_state: u32 },
    #[error("Unknown property key {property_key:?}")]
    UnknownPropertyKey { property_key: PROPERTYKEY },
    #[error("Unknown property store access {access:?}")]
    UnknownPropertyStoreAccess { access: STGM },
    #[error("Unknown role {role:?}")]
    UnknownRole { role: ERole },
    #[error("Unknown runtime mode {runtime_mode:?}")]
    UnknownRuntimeMode { runtime_mode: COINIT },
    #[error("Expected {expected_type:?}, found {found_type:?}")]
    UnexpectedPropertyType {
        expected_type: PropertyType,
        found_type: PropertyType,
    },
}
