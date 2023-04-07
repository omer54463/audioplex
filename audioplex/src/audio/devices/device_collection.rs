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
    pub(crate) fn get_count(&self) -> Result<usize, Error> {
        unsafe { self.raw_interface.GetCount() }
            .map(|device_count| device_count as usize)
            .map_err(Error::from)
    }

    pub(crate) fn get(&self, index: usize) -> Result<InterfaceWrapper<Device>, Error> {
        unsafe { self.raw_interface.Item(index as u32) }
            .map(|raw_interface| self.runtime.wrap_instance(raw_interface))
            .map_err(Error::from)
    }
}
