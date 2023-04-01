use crate::com::{interface::Interface, interface_wrapper::InterfaceWrapper, runtime::Runtime};
use crate::{audio::device::device::Device, error::Error};
use windows::Win32::Media::Audio::IMMDeviceCollection;

pub(crate) struct DeviceCollection<'a> {
    runtime: &'a Runtime,
    unsafe_interface: IMMDeviceCollection,
}

impl<'a> Interface<'a> for DeviceCollection<'a> {
    type UnsafeInterface = IMMDeviceCollection;

    fn new(runtime: &'a Runtime, unsafe_interface: Self::UnsafeInterface) -> Self {
        Self {
            runtime,
            unsafe_interface,
        }
    }
}

impl<'a> DeviceCollection<'a> {
    pub(crate) fn get_count(&self) -> Result<u32, Error> {
        unsafe { self.unsafe_interface.GetCount() }
            .map(|count| count)
            .map_err(Error::from)
    }

    pub(crate) fn get_device(&self, index: u32) -> Result<InterfaceWrapper<Device>, Error> {
        unsafe { self.unsafe_interface.Item(index) }
            .map(|unsafe_interface| self.runtime.wrap_instance(unsafe_interface))
            .map_err(Error::from)
    }
}
