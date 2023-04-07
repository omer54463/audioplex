use crate::audio::sessions::session_manager_event::SessionManagerEvent;
use crate::error::Error;
use std::sync::mpsc::Sender;
use windows::core::{implement, Interface};
use windows::Win32::Media::Audio::{
    IAudioSessionControl, IAudioSessionControl2, IAudioSessionNotification,
    IAudioSessionNotification_Impl,
};

#[implement(IAudioSessionNotification)]
pub(crate) struct SessionManagerEventClient {
    sender: Sender<SessionManagerEvent>,
}

impl SessionManagerEventClient {
    pub(crate) fn new(sender: Sender<SessionManagerEvent>) -> Self {
        Self { sender }
    }

    fn on_session_created(&self, raw_session: &Option<IAudioSessionControl>) -> Result<(), Error> {
        match raw_session {
            Some(raw_session) => raw_session
                .cast::<IAudioSessionControl2>()
                .and_then(|raw_session| unsafe { raw_session.GetProcessId() })
                .map(|process_id| process_id as usize)
                .map(|process_id| SessionManagerEvent::Add { process_id })
                .map_err(Error::from)
                .and_then(|session_manager_event| {
                    self.sender.send(session_manager_event).map_err(Error::from)
                }),
            None => Ok(()),
        }
    }
}

impl IAudioSessionNotification_Impl for SessionManagerEventClient {
    fn OnSessionCreated(
        &self,
        raw_session: &Option<IAudioSessionControl>,
    ) -> windows::core::Result<()> {
        self.on_session_created(raw_session)
            .map_err(|error| error.into())
    }
}
