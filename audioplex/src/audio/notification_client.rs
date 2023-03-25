use windows::core::{implement, PCWSTR};
use windows::Win32::Media::Audio::{
    EDataFlow, ERole, IMMNotificationClient, IMMNotificationClient_Impl,
};
use windows::Win32::UI::Shell::PropertiesSystem::PROPERTYKEY;

#[implement(IMMNotificationClient)]
pub(crate) struct NotificationClient {}

impl IMMNotificationClient_Impl for NotificationClient {
    fn OnDeviceStateChanged(
        &self,
        device_id: &PCWSTR,
        device_state: u32,
    ) -> windows::core::Result<()> {
        todo!()
    }

    fn OnDeviceAdded(&self, device_id: &PCWSTR) -> windows::core::Result<()> {
        todo!()
    }

    fn OnDeviceRemoved(&self, device_id: &PCWSTR) -> windows::core::Result<()> {
        todo!()
    }

    fn OnDefaultDeviceChanged(
        &self,
        data_flow: EDataFlow,
        role: ERole,
        device_id: &PCWSTR,
    ) -> windows::core::Result<()> {
        todo!()
    }

    fn OnPropertyValueChanged(
        &self,
        device_id: &PCWSTR,
        property_key: &PROPERTYKEY,
    ) -> windows::core::Result<()> {
        todo!()
    }
}
