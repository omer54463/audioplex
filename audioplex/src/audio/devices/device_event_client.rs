use crate::audio::devices::device_event::DeviceEvent;
use crate::audio::properties::property_key::PropertyKey;
use crate::error::Error;
use std::sync::mpsc::Sender;
use windows::core::{implement, PCWSTR};
use windows::Win32::Media::Audio::{
    EDataFlow, ERole, IMMNotificationClient, IMMNotificationClient_Impl,
};
use windows::Win32::UI::Shell::PropertiesSystem::PROPERTYKEY;

#[implement(IMMNotificationClient)]
pub(crate) struct DeviceEventClient {
    sender: Sender<DeviceEvent>,
}

impl DeviceEventClient {
    pub(crate) fn new(sender: Sender<DeviceEvent>) -> Self {
        Self { sender }
    }

    fn on_device_state_changed(&self, device_id: &PCWSTR) -> Result<(), Error> {
        let device_id = unsafe { device_id.to_string() }?;
        self.sender
            .send(DeviceEvent::StateChange { device_id })
            .map_err(Error::from)
    }

    fn on_device_added(&self, device_id: &PCWSTR) -> Result<(), Error> {
        let device_id = unsafe { device_id.to_string() }?;
        self.sender
            .send(DeviceEvent::Add { device_id })
            .map_err(Error::from)
    }

    fn on_device_removed(&self, device_id: &PCWSTR) -> Result<(), Error> {
        let device_id = unsafe { device_id.to_string() }?;
        self.sender
            .send(DeviceEvent::Remove { device_id })
            .map_err(Error::from)
    }

    fn on_property_value_changed(
        &self,
        device_id: &PCWSTR,
        property_key: &PROPERTYKEY,
    ) -> Result<(), Error> {
        let device_id = unsafe { device_id.to_string() }?;

        let device_event = match (*property_key).try_into() {
            Ok(PropertyKey::DeviceName) => Ok(Some(DeviceEvent::NameChange { device_id })),
            Ok(PropertyKey::IconPath) => Ok(Some(DeviceEvent::IconChange { device_id })),
            Ok(PropertyKey::DeviceDescription) => {
                Ok(Some(DeviceEvent::DescriptionChange { device_id }))
            }
            Err(error) => Err(error),
        };

        match device_event {
            Ok(Some(device_event)) => self.sender.send(device_event).map_err(Error::from),
            Ok(None) => Ok(()),
            Err(error) => Err(error),
        }
    }
}

impl IMMNotificationClient_Impl for DeviceEventClient {
    fn OnDeviceStateChanged(
        &self,
        device_id: &PCWSTR,
        _device_state: u32,
    ) -> windows::core::Result<()> {
        self.on_device_state_changed(&device_id)
            .map_err(|error| error.into())
    }

    fn OnDeviceAdded(&self, device_id: &PCWSTR) -> windows::core::Result<()> {
        self.on_device_added(&device_id)
            .map_err(|error| error.into())
    }

    fn OnDeviceRemoved(&self, device_id: &PCWSTR) -> windows::core::Result<()> {
        self.on_device_removed(&device_id)
            .map_err(|error| error.into())
    }

    fn OnDefaultDeviceChanged(
        &self,
        _data_flow: EDataFlow,
        _role: ERole,
        _device_id: &PCWSTR,
    ) -> windows::core::Result<()> {
        Ok(())
    }

    fn OnPropertyValueChanged(
        &self,
        device_id: &PCWSTR,
        property_key: &PROPERTYKEY,
    ) -> windows::core::Result<()> {
        self.on_property_value_changed(device_id, property_key)
            .map_err(|error| error.into())
    }
}
