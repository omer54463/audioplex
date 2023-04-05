use crate::com::{interface::Interface, interface_wrapper::InterfaceWrapper, runtime::Runtime};
use crate::{audio::devices::device::Device, error::Error};
use windows::Win32::Media::Audio::IMMDeviceCollection;

pub(crate) struct DeviceCollection<'a> {
    runtime: &'a Runtime,
    raw_interface: IMMDeviceCollection,
}

impl<'a> Interface<'a> for DeviceCollection<'a> {
    type RawInterface = IMMDeviceCollection;

    fn new(runtime: &'a Runtime, raw_interface: Self::RawInterface) -> Self {
        Self {
            runtime,
            raw_interface,
        }
    }
}

impl<'a> DeviceCollection<'a> {
    pub(crate) fn get_count(&self) -> Result<u32, Error> {
        unsafe { self.raw_interface.GetCount() }.map_err(Error::from)
    }

    pub(crate) fn get_device(&self, index: u32) -> Result<InterfaceWrapper<Device>, Error> {
        unsafe { self.raw_interface.Item(index) }
            .map(|raw_interface| self.runtime.wrap_instance(raw_interface))
            .map_err(Error::from)
    }
}
