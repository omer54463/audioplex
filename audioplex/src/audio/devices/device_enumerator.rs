use crate::audio::devices::device::Device;
use crate::audio::role::Role;
use crate::com::creatable_interface::CreatableInterface;
use crate::com::{interface::Interface, interface_wrapper::InterfaceWrapper, runtime::Runtime};
use crate::{
    audio::data_flow::DataFlow, audio::devices::device_collection::DeviceCollection,
    audio::devices::device_state::DeviceState, error::Error,
};
use windows::core::PCWSTR;
use windows::Win32::Media::Audio::{IMMDeviceEnumerator, MMDeviceEnumerator};

pub(crate) struct DeviceEnumerator<'a> {
    runtime: &'a Runtime,
    unsafe_interface: IMMDeviceEnumerator,
}

impl<'a> Interface<'a> for DeviceEnumerator<'a> {
    type UnsafeInterface = IMMDeviceEnumerator;

    fn new(runtime: &'a Runtime, unsafe_interface: Self::UnsafeInterface) -> Self {
        Self {
            runtime,
            unsafe_interface,
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
            self.unsafe_interface
                .EnumAudioEndpoints(data_flow.into(), device_state.into())
        }
        .map(|unsafe_interface| self.runtime.wrap_instance(unsafe_interface))
        .map_err(Error::from)
    }

    pub(crate) fn get_default_device(
        &self,
        data_flow: DataFlow,
        role: Role,
    ) -> Result<InterfaceWrapper<Device>, Error> {
        unsafe {
            self.unsafe_interface
                .GetDefaultAudioEndpoint(data_flow.into(), role.into())
        }
        .map(|unsafe_interface| self.runtime.wrap_instance(unsafe_interface))
        .map_err(Error::from)
    }

    pub(crate) fn get_device(&self, device_id: String) -> Result<InterfaceWrapper<Device>, Error> {
        let device_id: Vec<_> = device_id.encode_utf16().chain([0]).collect();
        unsafe { self.unsafe_interface.GetDevice(PCWSTR(device_id.as_ptr())) }
            .map(|unsafe_interface| self.runtime.wrap_instance(unsafe_interface))
            .map_err(Error::from)
    }
}
