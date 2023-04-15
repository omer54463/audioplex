use std::{
    ops::Deref,
    sync::mpsc::{channel, Receiver},
};

use windows::Win32::Media::Audio::IAudioSessionEvents;

use crate::error::Error;

use super::{
    session::Session, session_event::SessionEvent, session_event_client::SessionEventClient,
};

pub(crate) struct SessionEventStream<'a> {
    session: &'a Session,
    event_client: IAudioSessionEvents,
    receiver: Receiver<SessionEvent<'a>>,
}

impl<'a> SessionEventStream<'a> {
    pub(crate) fn new(session: &'a Session) -> Result<Self, Error> {
        let (sender, receiver) = channel();

        let session_event_stream = Self {
            session,
            event_client: SessionEventClient::new(session.get_id(), sender).into(),
            receiver,
        };

        unsafe {
            session_event_stream
                .session
                .register_event_client(&session_event_stream.event_client)
        }
        .map(|_| session_event_stream)
    }
}

impl<'a> Deref for SessionEventStream<'a> {
    type Target = Receiver<SessionEvent<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.receiver
    }
}

impl<'a> Drop for SessionEventStream<'a> {
    fn drop(&mut self) {
        unsafe { self.session.unregister_event_client(&self.event_client) }
            .expect("Could not unregister session manager event client")
    }
}
