use crate::audio::sessions::session_control::SessionControl;
use crate::{
    com::{interface::Interface, interface_wrapper::InterfaceWrapper, runtime::Runtime},
    error::Error,
};
use windows::Win32::Media::Audio::IAudioSessionEnumerator;

pub(crate) struct SessionEnumerator<'a> {
    runtime: &'a Runtime,
    raw_interface: IAudioSessionEnumerator,
}

impl<'a> Interface<'a> for SessionEnumerator<'a> {
    type RawInterface = IAudioSessionEnumerator;

    fn new(runtime: &'a Runtime, raw_interface: Self::RawInterface) -> Self {
        Self {
            runtime,
            raw_interface,
        }
    }
}

impl<'a> SessionEnumerator<'a> {
    pub(crate) fn get_count(&self) -> Result<i32, Error> {
        unsafe { self.raw_interface.GetCount() }.map_err(Error::from)
    }

    pub(crate) fn get_session_control(
        &self,
        index: i32,
    ) -> Result<InterfaceWrapper<SessionControl>, Error> {
        unsafe { self.raw_interface.GetSession(index) }
            .map(|raw_interface| self.runtime.wrap_instance(raw_interface))
            .map_err(Error::from)
    }
}
