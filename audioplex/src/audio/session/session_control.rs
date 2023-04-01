use crate::audio::session::{
    session_extended_control::SessionExtendedControl, session_state::SessionState,
};
use crate::{
    com::{interface::Interface, interface_wrapper::InterfaceWrapper, runtime::Runtime},
    error::Error,
};
use windows::{core::Interface as _, Win32::Media::Audio::IAudioSessionControl};

pub(crate) struct SessionControl<'a> {
    runtime: &'a Runtime,
    unsafe_interface: IAudioSessionControl,
}

impl<'a> Interface<'a> for SessionControl<'a> {
    type UnsafeInterface = IAudioSessionControl;

    fn new(runtime: &'a Runtime, unsafe_interface: Self::UnsafeInterface) -> Self {
        Self {
            runtime,
            unsafe_interface,
        }
    }
}

impl<'a> SessionControl<'a> {
    pub(crate) fn get_display_name(&self) -> Result<String, Error> {
        unsafe { self.unsafe_interface.GetDisplayName() }
            .map_err(Error::from)
            .and_then(|id| unsafe { id.to_string() }.map_err(Error::from))
    }

    pub(crate) fn get_state(&self) -> Result<SessionState, Error> {
        unsafe { self.unsafe_interface.GetState() }
            .map_err(Error::from)
            .and_then(|state| state.try_into())
    }

    pub(crate) fn get_extended_control(
        &self,
    ) -> Result<InterfaceWrapper<SessionExtendedControl>, Error> {
        self.unsafe_interface
            .cast()
            .map(|unsafe_interface| self.runtime.wrap_instance(unsafe_interface))
            .map_err(Error::from)
    }
}
