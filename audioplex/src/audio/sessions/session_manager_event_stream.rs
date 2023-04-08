use crate::audio::sessions::session_manager::SessionManager;
use crate::audio::sessions::session_manager_event::SessionManagerEvent;
use crate::audio::sessions::session_manager_event_client::SessionManagerEventClient;
use crate::com::runtime::Runtime;
use crate::error::Error;
use std::ops::Deref;
use std::sync::mpsc::{channel, Receiver};
use windows::Win32::Media::Audio::IAudioSessionNotification;

pub(crate) struct SessionManagerEventStream<'a> {
    session_manager: &'a SessionManager<'a>,
    session_manager_event_client: IAudioSessionNotification,
    session_event_receiver: Receiver<SessionManagerEvent<'a>>,
}

impl<'a> SessionManagerEventStream<'a> {
    pub(crate) fn new(
        runtime: &'a Runtime,
        session_manager: &'a SessionManager<'a>,
    ) -> Result<Self, Error> {
        let (sender, receiver) = channel();

        let session_manager_event_stream = Self {
            session_manager,
            session_manager_event_client: SessionManagerEventClient::new(runtime, sender).into(),
            session_event_receiver: receiver,
        };

        unsafe {
            session_manager_event_stream
                .session_manager
                .register_event_client(&session_manager_event_stream.session_manager_event_client)
        }
        .map(|_| session_manager_event_stream)
    }
}

impl<'a> Deref for SessionManagerEventStream<'a> {
    type Target = Receiver<SessionManagerEvent<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.session_event_receiver
    }
}

impl<'a> Drop for SessionManagerEventStream<'a> {
    fn drop(&mut self) {
        unsafe {
            self.session_manager
                .unregister_event_client(&self.session_manager_event_client)
        }
        .expect("Could not unregister session manager event client")
    }
}
