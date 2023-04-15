use std::{
    ops::Deref,
    sync::mpsc::{channel, Receiver},
};

use windows::Win32::Media::Audio::IAudioSessionNotification;

use crate::{com::runtime::Runtime, error::Error};

use super::{
    session_event::SessionEvent, session_manager::SessionManager,
    session_manager_event_client::SessionManagerEventClient,
};

pub(crate) struct SessionManagerEventStream<'a> {
    session_manager: &'a SessionManager<'a>,
    event_client: IAudioSessionNotification,
    receiver: Receiver<SessionEvent<'a>>,
}

impl<'a> SessionManagerEventStream<'a> {
    pub(crate) fn new(
        runtime: &'a Runtime,
        session_manager: &'a SessionManager<'a>,
    ) -> Result<Self, Error> {
        let (sender, receiver) = channel();

        let session_manager_event_stream = Self {
            session_manager,
            event_client: SessionManagerEventClient::new(runtime, sender).into(),
            receiver,
        };

        unsafe {
            session_manager_event_stream
                .session_manager
                .register_event_client(&session_manager_event_stream.event_client)
        }
        .map(|_| session_manager_event_stream)
    }
}

impl<'a> Deref for SessionManagerEventStream<'a> {
    type Target = Receiver<SessionEvent<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.receiver
    }
}

impl<'a> Drop for SessionManagerEventStream<'a> {
    fn drop(&mut self) {
        unsafe {
            self.session_manager
                .unregister_event_client(&self.event_client)
        }
        .expect("Could not unregister session manager event client")
    }
}
