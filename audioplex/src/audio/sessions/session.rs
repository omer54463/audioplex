use uuid::Uuid;
use windows::{
    core::Vtable,
    Win32::{
        Foundation::{S_FALSE, S_OK},
        Media::Audio::{IAudioSessionControl2, IAudioSessionEvents},
    },
};

use crate::{
    com::{interface::Interface, runtime::Runtime},
    error::Error,
};

use super::{sessio_event_stream::SessionEventStream, session_state::SessionState};

pub(crate) struct Session {
    id: Uuid,
    raw_interface: IAudioSessionControl2,
}

impl<'a> Interface<'a> for Session {
    type RawInterface = IAudioSessionControl2;

    fn new(_runtime: &'a Runtime, raw_interface: Self::RawInterface) -> Self {
        Self {
            id: Uuid::new_v4(),
            raw_interface,
        }
    }
}

impl<'a> Session {
    pub(crate) fn get_id(&self) -> String {
        format!("{}", self.id.as_braced())
    }

    pub(crate) fn get_display_name(&self) -> Result<String, Error> {
        unsafe { self.raw_interface.GetDisplayName() }
            .map_err(Error::from)
            .and_then(|id| unsafe { id.to_string() }.map_err(Error::from))
    }

    pub(crate) fn get_icon_path(&self) -> Result<String, Error> {
        unsafe { self.raw_interface.GetIconPath() }
            .map_err(Error::from)
            .and_then(|id| unsafe { id.to_string() }.map_err(Error::from))
    }

    pub(crate) fn get_state(&self) -> Result<SessionState, Error> {
        unsafe { self.raw_interface.GetState() }
            .map_err(Error::from)
            .and_then(|state| state.try_into())
    }

    pub(crate) fn get_process_id(&self) -> Result<usize, Error> {
        unsafe { self.raw_interface.GetProcessId() }
            .map(|process_id| process_id as usize)
            .map_err(Error::from)
    }

    pub(crate) fn is_system(&self) -> Result<bool, Error> {
        let hresult = unsafe {
            (Vtable::vtable(&self.raw_interface).IsSystemSoundsSession)(Vtable::as_raw(
                &self.raw_interface,
            ))
        };

        match hresult {
            S_OK => Ok(true),
            S_FALSE => Ok(false),
            _ if hresult.is_err() => Err(Error::Windows(hresult.into())),
            _ => Err(Error::UnexpectedHResult { hresult }),
        }
    }

    pub(crate) fn get_event_stream(&'a self) -> Result<SessionEventStream<'a>, Error> {
        SessionEventStream::new(self)
    }

    pub(crate) unsafe fn register_event_client(
        &self,
        event_client: &IAudioSessionEvents,
    ) -> Result<(), Error> {
        self.raw_interface
            .RegisterAudioSessionNotification(event_client)
            .map_err(Error::from)
    }

    pub(crate) unsafe fn unregister_event_client(
        &self,
        event_client: &IAudioSessionEvents,
    ) -> Result<(), Error> {
        self.raw_interface
            .UnregisterAudioSessionNotification(event_client)
            .map_err(Error::from)
    }
}
