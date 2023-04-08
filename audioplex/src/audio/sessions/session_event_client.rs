use crate::{audio::sessions::session_event::SessionEvent, error::Error};
use audioplex_implement::implement;
use std::sync::mpsc::Sender;
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

#[implement(IAudioSessionEvents)]
pub(crate) struct SessionEventClient {
    sender: Sender<SessionEvent>,
}

impl SessionEventClient {
    pub(crate) fn new(sender: Sender<SessionEvent>) -> Self {
        Self { sender }
    }

    fn on_display_name_changed(&self, display_name: &PCWSTR) -> Result<(), Error> {
        unsafe { display_name.to_string() }
            .map_err(Error::from)
            .and_then(|display_name| {
                self.sender
                    .send(SessionEvent::NameChange { display_name })
                    .map_err(Error::from)
            })
    }

    fn on_icon_path_changed(&self, icon_path: &PCWSTR) -> Result<(), Error> {
        unsafe { icon_path.to_string() }
            .map_err(Error::from)
            .and_then(|icon_path| {
                self.sender
                    .send(SessionEvent::IconChange { icon_path })
                    .map_err(Error::from)
            })
    }

    fn on_simple_volume_changed(&self, volume: f32, mute: BOOL) -> Result<(), Error> {
        let mute = mute.as_bool();
        self.sender
            .send(SessionEvent::VolumeChange { volume, mute })
            .map_err(Error::from)
    }

    fn on_state_changed(&self, session_state: AudioSessionState) -> Result<(), Error> {
        session_state.try_into().and_then(|session_state| {
            self.sender
                .send(SessionEvent::StateChange { session_state })
                .map_err(Error::from)
        })
    }

    fn on_session_disconnected(&self) -> Result<(), Error> {
        self.sender.send(SessionEvent::Remove).map_err(Error::from)
    }
}

impl IAudioSessionEvents_Impl for SessionEventClient {
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
