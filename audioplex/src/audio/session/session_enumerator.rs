use crate::audio::session::session_control::SessionControl;
use crate::{
    com::{interface::Interface, interface_wrapper::InterfaceWrapper, runtime::Runtime},
    error::Error,
};
use windows::Win32::Media::Audio::IAudioSessionEnumerator;

pub(crate) struct SessionEnumerator<'a> {
    runtime: &'a Runtime,
    unsafe_interface: IAudioSessionEnumerator,
}

impl<'a> Interface<'a> for SessionEnumerator<'a> {
    type UnsafeInterface = IAudioSessionEnumerator;

    fn new(runtime: &'a Runtime, unsafe_interface: Self::UnsafeInterface) -> Self {
        Self {
            runtime,
            unsafe_interface,
        }
    }
}

impl<'a> SessionEnumerator<'a> {
    pub(crate) fn get_count(&self) -> Result<i32, Error> {
        unsafe { self.unsafe_interface.GetCount() }
            .map(|count| count)
            .map_err(Error::from)
    }

    pub(crate) fn get_session_control(
        &self,
        index: i32,
    ) -> Result<InterfaceWrapper<SessionControl>, Error> {
        unsafe { self.unsafe_interface.GetSession(index) }
            .map(|unsafe_interface| self.runtime.wrap_instance(unsafe_interface))
            .map_err(Error::from)
    }
}
