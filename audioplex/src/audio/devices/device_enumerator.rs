use std::sync::mpsc::Sender;

use crate::audio::devices::device::Device;
use crate::com::creatable_interface::CreatableInterface;
use crate::com::{interface::Interface, interface_wrapper::InterfaceWrapper, runtime::Runtime};
use crate::event::Event;
use crate::{
    audio::data_flow::DataFlow, audio::devices::device_collection::DeviceCollection,
    audio::devices::device_state::DeviceState, error::Error,
};
use windows::core::PCWSTR;
use windows::Win32::Media::Audio::{
    IMMDeviceEnumerator, IMMNotificationClient, MMDeviceEnumerator,
};

use super::device_event_client::DeviceEventClient;

pub(crate) struct DeviceEnumerator<'a> {
    runtime: &'a Runtime,
    raw_interface: IMMDeviceEnumerator,
    raw_notification_client: Option<IMMNotificationClient>,
}

impl<'a> Interface<'a> for DeviceEnumerator<'a> {
    type RawInterface = IMMDeviceEnumerator;

    fn new(runtime: &'a Runtime, raw_interface: Self::RawInterface) -> Self {
        Self {
            runtime,
            raw_interface,
            raw_notification_client: None,
        }
    }
}

impl<'a> CreatableInterface<'a> for DeviceEnumerator<'a> {
    fn get_guid() -> windows::core::GUID {
        MMDeviceEnumerator
    }
}

impl<'a> DeviceEnumerator<'a> {
    pub(crate) fn get_device_collection(
        &self,
        data_flow: DataFlow,
        device_state: DeviceState,
    ) -> Result<InterfaceWrapper<DeviceCollection>, Error> {
        unsafe {
            self.raw_interface
                .EnumAudioEndpoints(data_flow.into(), device_state.into())
        }
        .map(|raw_interface| self.runtime.wrap_instance(raw_interface))
        .map_err(Error::from)
    }

    pub(crate) fn get_device(&self, device_id: String) -> Result<InterfaceWrapper<Device>, Error> {
        let device_id: Vec<_> = device_id.encode_utf16().chain([0]).collect();
        unsafe { self.raw_interface.GetDevice(PCWSTR(device_id.as_ptr())) }
            .map(|raw_interface| self.runtime.wrap_instance(raw_interface))
            .map_err(Error::from)
    }

    pub(crate) fn start_events(&mut self, sender: Sender<Event>) -> Result<(), Error> {
        self.stop_events()?;
        self.raw_notification_client = Some(DeviceEventClient::new(sender).into());
        unsafe {
            self.raw_interface.RegisterEndpointNotificationCallback(
                self.raw_notification_client.as_ref().unwrap(),
            )
        }
        .map_err(Error::from)
    }

    pub(crate) fn stop_events(&mut self) -> Result<(), Error> {
        match &self.raw_notification_client {
            Some(raw_notification_client) => unsafe {
                self.raw_interface
                    .UnregisterEndpointNotificationCallback(raw_notification_client)
                    .map_err(Error::from)
            },
            None => Ok(()),
        }
        .and_then(|_| {
            self.raw_notification_client = None;
            Ok(())
        })
    }
}

impl<'a> Drop for DeviceEnumerator<'a> {
    fn drop(&mut self) {
        self.stop_events()
            .expect("Could not unregister device event client");
    }
}
