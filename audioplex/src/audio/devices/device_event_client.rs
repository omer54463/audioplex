use crate::audio::devices::device_enumerator::DeviceEnumerator;
use crate::audio::devices::{device::Device, device_event::DeviceEvent};
use crate::audio::properties::property_key::PropertyKey;
use crate::com::interface_wrapper::InterfaceWrapper;
use crate::error::Error;
use audioplex_implement::implement;
use std::sync::mpsc::Sender;
use windows::core::PCWSTR;
use windows::Win32::Media::Audio::{
    EDataFlow, ERole, IMMNotificationClient, IMMNotificationClient_Impl,
};
use windows::Win32::UI::Shell::PropertiesSystem::PROPERTYKEY;

#[implement(IMMNotificationClient)]
pub(crate) struct DeviceEventClient<'a> {
    device_enumerator: &'a DeviceEnumerator<'a>,
    sender: Sender<DeviceEvent<'a>>,
}

impl<'a> DeviceEventClient<'a> {
    pub(crate) fn new(
        device_enumerator: &'a DeviceEnumerator<'a>,
        sender: Sender<DeviceEvent<'a>>,
    ) -> Self {
        Self {
            device_enumerator,
            sender,
        }
    }

    fn get_device(&self, device_id: &PCWSTR) -> Result<InterfaceWrapper<'a, Device<'a>>, Error> {
        unsafe { device_id.to_string() }
            .map_err(Error::from)
            .and_then(|device_id| self.device_enumerator.get(device_id))
    }

    fn on_device_state_changed(&self, device_id: &PCWSTR) -> Result<(), Error> {
        self.get_device(device_id).and_then(|device| {
            self.sender
                .send(DeviceEvent::StateChange { device })
                .map_err(Error::from)
        })
    }

    fn on_device_added(&self, device_id: &PCWSTR) -> Result<(), Error> {
        self.get_device(device_id).and_then(|device| {
            self.sender
                .send(DeviceEvent::Add { device })
                .map_err(Error::from)
        })
    }

    fn on_device_removed(&self, device_id: &PCWSTR) -> Result<(), Error> {
        self.get_device(device_id).and_then(|device| {
            self.sender
                .send(DeviceEvent::Remove { device })
                .map_err(Error::from)
        })
    }

    fn on_property_value_changed(
        &self,
        device_id: &PCWSTR,
        property_key: &PROPERTYKEY,
    ) -> Result<(), Error> {
        let device = self.get_device(device_id)?;

        let device_event = match (*property_key).try_into() {
            Ok(PropertyKey::DeviceName) => Ok(Some(DeviceEvent::NameChange { device })),
            Ok(PropertyKey::IconPath) => Ok(Some(DeviceEvent::IconChange { device })),
            Ok(PropertyKey::DeviceDescription) => {
                Ok(Some(DeviceEvent::DescriptionChange { device }))
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

impl<'a> IMMNotificationClient_Impl for DeviceEventClient<'a> {
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
