use crate::audio::devices::device_enumerator::DeviceEnumerator;
use crate::audio::devices::device_event::DeviceEvent;
use crate::audio::properties::property_key::PropertyKey;
use crate::audio::properties::property_store_access::PropertyStoreAccess;
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

    fn on_device_state_changed(&self, device_id: &PCWSTR, device_state: u32) -> Result<(), Error> {
        let device_id = unsafe { device_id.to_string() }.map_err(Error::from)?;
        let device_state = device_state.try_into()?;
        let device_event = DeviceEvent::StateChange {
            device_id,
            device_state,
        };

        self.sender.send(device_event).map_err(Error::from)
    }

    fn on_device_added(&self, device_id: &PCWSTR) -> Result<(), Error> {
        let device_id = unsafe { device_id.to_string() }.map_err(Error::from)?;
        let device = self.device_enumerator.get(&device_id)?;
        let device_event = DeviceEvent::Add { device_id, device };

        self.sender.send(device_event).map_err(Error::from)
    }

    fn on_device_removed(&self, device_id: &PCWSTR) -> Result<(), Error> {
        let device_id = unsafe { device_id.to_string() }.map_err(Error::from)?;
        let device_event = DeviceEvent::Remove { device_id };

        self.sender.send(device_event).map_err(Error::from)
    }

    fn on_property_value_changed(
        &self,
        device_id: &PCWSTR,
        property_key: &PROPERTYKEY,
    ) -> Result<(), Error> {
        let device_id = unsafe { device_id.to_string() }.map_err(Error::from)?;
        let device = self.device_enumerator.get(&device_id)?;
        let property_store = device.get_property_store(PropertyStoreAccess::Read)?;

        let device_event = match (*property_key).try_into() {
            Ok(PropertyKey::DeviceName) => {
                let display_name = property_store.get_string(PropertyKey::DeviceName)?;
                Ok(Some(DeviceEvent::NameChange {
                    device_id,
                    display_name,
                }))
            }
            Ok(PropertyKey::IconPath) => {
                let icon_path = property_store.get_string(PropertyKey::IconPath)?;
                Ok(Some(DeviceEvent::IconChange {
                    device_id,
                    icon_path,
                }))
            }
            Ok(PropertyKey::DeviceDescription) => {
                let description = property_store.get_string(PropertyKey::DeviceDescription)?;
                Ok(Some(DeviceEvent::DescriptionChange {
                    device_id,
                    description,
                }))
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
        device_state: u32,
    ) -> windows::core::Result<()> {
        self.on_device_state_changed(&device_id, device_state)
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
