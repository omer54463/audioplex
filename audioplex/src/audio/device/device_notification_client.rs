use crate::audio::device::device_notification_callback::DeviceNotificationCallback;
use crate::audio::property::property_key::PropertyKey;
use crate::error::Error;
use windows::core::{implement, PCWSTR};
use windows::Win32::Media::Audio::{
    EDataFlow, ERole, IMMNotificationClient, IMMNotificationClient_Impl,
};
use windows::Win32::UI::Shell::PropertiesSystem::PROPERTYKEY;

#[implement(IMMNotificationClient)]
pub(crate) struct DeviceNotificationClient {
    callback: Box<dyn DeviceNotificationCallback>,
}

impl DeviceNotificationClient {
    fn new(callback: Box<dyn DeviceNotificationCallback>) -> Self {
        Self { callback }
    }

    fn on_device_state_changed(&self, device_id: &PCWSTR, device_state: u32) -> Result<(), Error> {
        let device_id = unsafe { device_id.to_string() }?;
        let device_state = device_state.try_into()?;
        self.callback.on_state_change(device_id, device_state);
        Ok(())
    }

    fn on_device_added(&self, device_id: &PCWSTR) -> Result<(), Error> {
        let device_id = unsafe { device_id.to_string() }?;
        self.callback.on_add(device_id);
        Ok(())
    }

    fn on_device_removed(&self, device_id: &PCWSTR) -> Result<(), Error> {
        let device_id = unsafe { device_id.to_string() }?;
        self.callback.on_remove(device_id);
        Ok(())
    }

    fn on_property_value_changed(
        &self,
        device_id: &PCWSTR,
        property_key: &PROPERTYKEY,
    ) -> Result<(), Error> {
        let device_id = unsafe { device_id.to_string() }?;
        match property_key.clone().try_into() {
            Ok(PropertyKey::DeviceName) => Ok(self.callback.on_name_change(device_id)),
            Ok(PropertyKey::IconPath) => Ok(self.callback.on_icon_change(device_id)),
            Ok(_) => Ok(()),
            Err(error) => Err(error),
        }
    }
}

impl IMMNotificationClient_Impl for DeviceNotificationClient {
    fn OnDeviceStateChanged(
        &self,
        device_id: &PCWSTR,
        device_state: u32,
    ) -> windows::core::Result<()> {
        self.on_device_state_changed(device_id, device_state)
            .map_err(|error| error.into())
    }

    fn OnDeviceAdded(&self, device_id: &PCWSTR) -> windows::core::Result<()> {
        self.on_device_added(device_id)
            .map_err(|error| error.into())
    }

    fn OnDeviceRemoved(&self, device_id: &PCWSTR) -> windows::core::Result<()> {
        self.on_device_removed(device_id)
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
