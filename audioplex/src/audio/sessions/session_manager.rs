use crate::audio::sessions::session_enumerator::SessionEnumerator;
use crate::audio::sessions::session_manager_event_stream::SessionManagerEventStream;
use crate::{
    com::{interface::Interface, interface_wrapper::InterfaceWrapper, runtime::Runtime},
    error::Error,
};
use windows::Win32::Media::Audio::{IAudioSessionManager2, IAudioSessionNotification};

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
    pub(crate) fn get_enumerator(
        &self,
    ) -> Result<InterfaceWrapper<'a, SessionEnumerator<'a>>, Error> {
        unsafe { self.raw_interface.GetSessionEnumerator() }
            .map(|raw_interface| self.runtime.wrap_instance(raw_interface))
            .map_err(Error::from)
    }

    pub(crate) fn get_event_stream(&'a self) -> Result<SessionManagerEventStream<'a>, Error> {
        self.get_enumerator()
            .and_then(|session_enumerator| session_enumerator.get_count())
            .and_then(|_| SessionManagerEventStream::new(self))
    }

    pub(crate) unsafe fn register_event_client(
        &self,
        event_client: &'a IAudioSessionNotification,
    ) -> Result<(), Error> {
        self.raw_interface
            .RegisterSessionNotification(event_client)
            .map_err(Error::from)
    }

    pub(crate) unsafe fn unregister_event_client(
        &self,
        notification_client: &'a IAudioSessionNotification,
    ) -> Result<(), Error> {
        self.raw_interface
            .UnregisterSessionNotification(notification_client)
            .map_err(Error::from)
    }
}
