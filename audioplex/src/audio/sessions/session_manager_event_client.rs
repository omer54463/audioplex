use crate::audio::sessions::session_manager_event::SessionManagerEvent;
use crate::com::runtime::Runtime;
use crate::error::Error;
use audioplex_implement::implement;
use std::sync::mpsc::Sender;
use windows::core::Interface;
use windows::Win32::Media::Audio::{
    IAudioSessionControl, IAudioSessionControl2, IAudioSessionNotification,
    IAudioSessionNotification_Impl,
};

#[implement(IAudioSessionNotification)]
pub(crate) struct SessionManagerEventClient<'a> {
    runtime: &'a Runtime,
    sender: Sender<SessionManagerEvent<'a>>,
}

impl<'a> SessionManagerEventClient<'a> {
    pub(crate) fn new(runtime: &'a Runtime, sender: Sender<SessionManagerEvent<'a>>) -> Self {
        Self { runtime, sender }
    }

    fn on_session_created(
        &self,
        raw_interface: &Option<IAudioSessionControl>,
    ) -> Result<(), Error> {
        match raw_interface {
            Some(raw_interface) => raw_interface
                .cast::<IAudioSessionControl2>()
                .map(|raw_interface| self.runtime.wrap_instance(raw_interface))
                .map(|session| SessionManagerEvent::Add { session })
                .map_err(Error::from)
                .and_then(|session_manager_event| {
                    self.sender.send(session_manager_event).map_err(Error::from)
                }),
            None => Ok(()),
        }
    }
}

impl<'a> IAudioSessionNotification_Impl for SessionManagerEventClient<'a> {
    fn OnSessionCreated(
        &self,
        raw_interface: &Option<IAudioSessionControl>,
    ) -> windows::core::Result<()> {
        self.on_session_created(raw_interface)
            .map_err(|error| error.into())
    }
}
