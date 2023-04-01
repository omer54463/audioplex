use crate::audio::session_enumerator::SessionEnumerator;
use crate::{
    com::{interface::Interface, interface_wrapper::InterfaceWrapper, runtime::Runtime},
    error::Error,
};
use windows::Win32::Media::Audio::IAudioSessionManager2;

pub(crate) struct SessionManager<'a> {
    runtime: &'a Runtime,
    unsafe_interface: IAudioSessionManager2,
}

impl<'a> Interface<'a> for SessionManager<'a> {
    type UnsafeInterface = IAudioSessionManager2;

    fn new(runtime: &'a Runtime, unsafe_interface: Self::UnsafeInterface) -> Self {
        Self {
            runtime,
            unsafe_interface,
        }
    }
}

impl<'a> SessionManager<'a> {
    pub(crate) fn get_session_enumerator(
        &self,
    ) -> Result<InterfaceWrapper<'a, SessionEnumerator<'a>>, Error> {
        unsafe { self.unsafe_interface.GetSessionEnumerator() }
            .map(|unsafe_interface| self.runtime.wrap_instance(unsafe_interface))
            .map_err(Error::from)
    }
}
