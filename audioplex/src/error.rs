use crate::audio::properties::property_type::PropertyType;
use std::{string::FromUtf16Error, sync::mpsc::SendError};
use thiserror::Error;
use windows::{
    core::HRESULT,
    Win32::{
        Foundation::E_UNEXPECTED,
        Media::Audio::{AudioSessionState, EDataFlow, ERole},
        System::Com::{COINIT, STGM},
        UI::Shell::PropertiesSystem::PROPERTYKEY,
    },
};

#[derive(Error, Debug)]
pub(crate) enum Error {
    #[error("Windows error")]
    Windows(#[from] ::windows::core::Error),
    #[error("FromUtf16Error error")]
    FromUtf16(#[from] FromUtf16Error),
    #[error("Send error")]
    Send,

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
    #[error("Unknown session state {session_state:?}")]
    UnknownSessionState { session_state: AudioSessionState },

    #[error("Expected {expected_type:?}, found {found_type:?}")]
    UnexpectedPropertyType {
        expected_type: PropertyType,
        found_type: PropertyType,
    },
    #[error("Unexpected HResult {hresult}")]
    UnexpectedHResult { hresult: HRESULT },
}

impl<T> From<SendError<T>> for Error {
    fn from(_value: SendError<T>) -> Self {
        Self::Send
    }
}

impl From<Error> for windows::core::Error {
    fn from(value: Error) -> Self {
        Self::new(E_UNEXPECTED, value.to_string().into())
    }
}
