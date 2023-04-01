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

pub(crate) struct SessionExtendedControl<'a> {
    runtime: &'a Runtime,
    unsafe_interface: IAudioSessionControl2,
}

impl<'a> Interface<'a> for SessionExtendedControl<'a> {
    type UnsafeInterface = IAudioSessionControl2;

    fn new(runtime: &'a Runtime, unsafe_interface: Self::UnsafeInterface) -> Self {
        Self {
            runtime,
            unsafe_interface,
        }
    }
}

impl<'a> SessionExtendedControl<'a> {
    pub(crate) fn get_process_id(&self) -> Result<u32, Error> {
        unsafe { self.unsafe_interface.GetProcessId() }.map_err(Error::from)
    }

    pub(crate) fn is_system_sounds(&self) -> Result<bool, Error> {
        let hresult = unsafe {
            (Vtable::vtable(&self.unsafe_interface).IsSystemSoundsSession)(Vtable::as_raw(
                &self.unsafe_interface,
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
