use crate::audio::sessions::session::Session;
use crate::audio::sessions::session_event::SessionEvent;
use crate::com::interface_wrapper::InterfaceWrapper;
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
    sender: Sender<SessionEvent<'a>>,
}

impl<'a> SessionManagerEventClient<'a> {
    pub(crate) fn new(runtime: &'a Runtime, sender: Sender<SessionEvent<'a>>) -> Self {
        Self { runtime, sender }
    }

    fn on_session_created(
        &self,
        raw_interface: &Option<IAudioSessionControl>,
    ) -> Result<(), Error> {
        match raw_interface {
            Some(raw_interface) => {
                let session: InterfaceWrapper<Session> = raw_interface
                    .cast::<IAudioSessionControl2>()
                    .map(|raw_interface| self.runtime.wrap_instance(raw_interface))?;
                let session_id = session.get_id();

                let session_event = SessionEvent::Add {
                    session_id,
                    session,
                };

                self.sender.send(session_event).map_err(Error::from)
            }
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
