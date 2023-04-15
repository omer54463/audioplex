use std::sync::mpsc::Sender;

use audioplex_implement::implement;
use windows::{
    core::{GUID, PCWSTR},
    Win32::{
        Foundation::BOOL,
        Media::Audio::{
            AudioSessionDisconnectReason, AudioSessionState, IAudioSessionEvents,
            IAudioSessionEvents_Impl,
        },
    },
};

use crate::error::Error;

use super::session_event::SessionEvent;

#[implement(IAudioSessionEvents)]
pub(crate) struct SessionEventClient<'a> {
    session_id: String,
    sender: Sender<SessionEvent<'a>>,
}

impl<'a> SessionEventClient<'a> {
    pub(crate) fn new(session_id: String, sender: Sender<SessionEvent<'a>>) -> Self {
        Self { session_id, sender }
    }

    fn on_display_name_changed(&self, display_name: &PCWSTR) -> Result<(), Error> {
        let session_id = self.session_id.clone();
        let display_name = unsafe { display_name.to_string() }.map_err(Error::from)?;

        let session_event = SessionEvent::NameChange {
            session_id,
            display_name,
        };

        self.sender.send(session_event).map_err(Error::from)
    }

    fn on_icon_path_changed(&self, icon_path: &PCWSTR) -> Result<(), Error> {
        let session_id = self.session_id.clone();
        let icon_path = unsafe { icon_path.to_string() }.map_err(Error::from)?;

        let session_event = SessionEvent::IconChange {
            session_id,
            icon_path,
        };

        self.sender.send(session_event).map_err(Error::from)
    }

    fn on_simple_volume_changed(&self, volume: f32, mute: BOOL) -> Result<(), Error> {
        let session_id = self.session_id.clone();
        let mute = mute.as_bool();

        let session_event = SessionEvent::VolumeChange {
            session_id,
            volume,
            mute,
        };

        self.sender.send(session_event).map_err(Error::from)
    }

    fn on_state_changed(&self, session_state: AudioSessionState) -> Result<(), Error> {
        let session_id = self.session_id.clone();
        let session_state = session_state.try_into()?;

        let session_event = SessionEvent::StateChange {
            session_id,
            session_state,
        };

        self.sender.send(session_event).map_err(Error::from)
    }

    fn on_session_disconnected(&self) -> Result<(), Error> {
        let session_id = self.session_id.clone();

        let session_event = SessionEvent::Remove { session_id };

        self.sender.send(session_event).map_err(Error::from)
    }
}

impl<'a> IAudioSessionEvents_Impl for SessionEventClient<'a> {
    fn OnDisplayNameChanged(
        &self,
        display_name: &PCWSTR,
        _event_context: *const GUID,
    ) -> windows::core::Result<()> {
        self.on_display_name_changed(display_name)
            .map_err(|error| error.into())
    }

    fn OnIconPathChanged(
        &self,
        icon_path: &PCWSTR,
        _event_context: *const GUID,
    ) -> windows::core::Result<()> {
        self.on_icon_path_changed(icon_path)
            .map_err(|error| error.into())
    }

    fn OnSimpleVolumeChanged(
        &self,
        volume: f32,
        mute: BOOL,
        _event_context: *const GUID,
    ) -> windows::core::Result<()> {
        self.on_simple_volume_changed(volume, mute)
            .map_err(|error| error.into())
    }

    fn OnChannelVolumeChanged(
        &self,
        _channel_count: u32,
        _new_channel_volume_array: *const f32,
        _changed_channel: u32,
        _event_context: *const GUID,
    ) -> windows::core::Result<()> {
        Ok(())
    }

    fn OnGroupingParamChanged(
        &self,
        _grouping_parameter: *const GUID,
        _event_context: *const GUID,
    ) -> windows::core::Result<()> {
        Ok(())
    }

    fn OnStateChanged(&self, session_state: AudioSessionState) -> windows::core::Result<()> {
        self.on_state_changed(session_state)
            .map_err(|error| error.into())
    }

    fn OnSessionDisconnected(
        &self,
        _reason: AudioSessionDisconnectReason,
    ) -> windows::core::Result<()> {
        self.on_session_disconnected().map_err(|error| error.into())
    }
}
