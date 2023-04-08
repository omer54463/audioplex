use crate::audio::sessions::session::Session;
use crate::audio::sessions::session_event::SessionEvent;
use crate::audio::sessions::session_event_client::SessionEventClient;
use crate::error::Error;
use std::ops::Deref;
use std::sync::mpsc::{channel, Receiver};
use windows::Win32::Media::Audio::IAudioSessionEvents;

pub(crate) struct SessionEventStream<'a> {
    session: &'a Session,
    session_event_client: IAudioSessionEvents,
    session_event_receiver: Receiver<SessionEvent>,
}

impl<'a> SessionEventStream<'a> {
    pub(crate) fn new(session: &'a Session) -> Result<Self, Error> {
        let (sender, receiver) = channel();

        let session_event_stream = Self {
            session,
            session_event_client: SessionEventClient::new(sender).into(),
            session_event_receiver: receiver,
        };

        unsafe {
            session_event_stream
                .session
                .register_event_client(&session_event_stream.session_event_client)
        }
        .map(|_| session_event_stream)
    }
}

impl<'a> Deref for SessionEventStream<'a> {
    type Target = Receiver<SessionEvent>;

    fn deref(&self) -> &Self::Target {
        &self.session_event_receiver
    }
}

impl<'a> Drop for SessionEventStream<'a> {
    fn drop(&mut self) {
        unsafe {
            self.session
                .unregister_event_client(&self.session_event_client)
        }
        .expect("Could not unregister session manager event client")
    }
}
