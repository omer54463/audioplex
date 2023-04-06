use crate::audio::devices::device_event::DeviceEvent;
use crate::audio::devices::{
    device_enumerator::DeviceEnumerator, device_event_client::DeviceEventClient,
};
use crate::error::Error;
use std::ops::Deref;
use std::sync::mpsc::{channel, Receiver};
use windows::Win32::Media::Audio::IMMNotificationClient;

pub(crate) struct DeviceEventStream<'a> {
    device_enumerator: &'a DeviceEnumerator<'a>,
    device_event_client: IMMNotificationClient,
    device_event_receiver: Receiver<DeviceEvent>,
}

impl<'a> DeviceEventStream<'a> {
    pub(crate) fn new(device_enumerator: &'a DeviceEnumerator<'a>) -> Result<Self, Error> {
        let (sender, receiver) = channel();

        let device_event_stream = Self {
            device_enumerator,
            device_event_client: DeviceEventClient::new(sender).into(),
            device_event_receiver: receiver,
        };

        unsafe {
            device_event_stream
                .device_enumerator
                .register_notification_client(&device_event_stream.device_event_client)
        }
        .map(|_| device_event_stream)
    }
}

impl<'a> Deref for DeviceEventStream<'a> {
    type Target = Receiver<DeviceEvent>;

    fn deref(&self) -> &Self::Target {
        &self.device_event_receiver
    }
}

impl<'a> Drop for DeviceEventStream<'a> {
    fn drop(&mut self) {
        unsafe {
            self.device_enumerator
                .unregister_notification_client(&self.device_event_client)
        }
        .expect("Could not unregister device event client")
    }
}