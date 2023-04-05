use crate::audio::sessions::session_enumerator::SessionEnumerator;
use crate::{
    com::{interface::Interface, interface_wrapper::InterfaceWrapper, runtime::Runtime},
    error::Error,
};
use windows::Win32::Media::Audio::IAudioSessionManager2;

pub(crate) struct SessionManager<'a> {
    runtime: &'a Runtime,
    raw_interface: IAudioSessionManager2,
}

impl<'a> Interface<'a> for SessionManager<'a> {
    type RawInterface = IAudioSessionManager2;

    fn new(runtime: &'a Runtime, raw_interface: Self::RawInterface) -> Self {
        Self {
            runtime,
            raw_interface,
        }
    }
}

impl<'a> SessionManager<'a> {
    pub(crate) fn get_session_enumerator(
        &self,
    ) -> Result<InterfaceWrapper<'a, SessionEnumerator<'a>>, Error> {
        unsafe { self.raw_interface.GetSessionEnumerator() }
            .map(|raw_interface| self.runtime.wrap_instance(raw_interface))
            .map_err(Error::from)
    }
}
