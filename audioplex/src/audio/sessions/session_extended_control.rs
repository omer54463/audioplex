use crate::{
    com::{interface::Interface, runtime::Runtime},
    error::Error,
};
use windows::{
    core::Vtable,
    Win32::{
        Foundation::{S_FALSE, S_OK},
        Media::Audio::IAudioSessionControl2,
    },
};

pub(crate) struct SessionExtendedControl {
    raw_interface: IAudioSessionControl2,
}

impl<'a> Interface<'a> for SessionExtendedControl {
    type RawInterface = IAudioSessionControl2;

    fn new(_runtime: &'a Runtime, raw_interface: Self::RawInterface) -> Self {
        Self { raw_interface }
    }
}

impl SessionExtendedControl {
    pub(crate) fn get_process_id(&self) -> Result<usize, Error> {
        unsafe { self.raw_interface.GetProcessId() }
            .map(|process_id| process_id as usize)
            .map_err(Error::from)
    }

    pub(crate) fn is_system(&self) -> Result<bool, Error> {
        let hresult = unsafe {
            (Vtable::vtable(&self.raw_interface).IsSystemSoundsSession)(Vtable::as_raw(
                &self.raw_interface,
            ))
        };

        match hresult {
            S_OK => Ok(true),
            S_FALSE => Ok(false),
            _ if let Err(error) = hresult.ok() => Err(Error::from(error)),
            _ => Err(Error::UnexpectedHResult { hresult })
        }
    }
}
