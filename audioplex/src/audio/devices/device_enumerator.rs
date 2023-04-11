use crate::audio::devices::device::Device;
use crate::audio::devices::device_event_stream::DeviceEventStream;
use crate::com::creatable_interface::CreatableInterface;
use crate::com::{interface::Interface, interface_wrapper::InterfaceWrapper, runtime::Runtime};
use crate::{
    audio::data_flow::DataFlow, audio::devices::device_collection::DeviceCollection,
    audio::devices::device_state::DeviceState, error::Error,
};
use windows::core::PCWSTR;
use windows::Win32::Media::Audio::{
    IMMDeviceEnumerator, IMMNotificationClient, MMDeviceEnumerator,
};

pub(crate) struct DeviceEnumerator<'a> {
    runtime: &'a Runtime,
    raw_interface: IMMDeviceEnumerator,
}

impl<'a> Interface<'a> for DeviceEnumerator<'a> {
    type RawInterface = IMMDeviceEnumerator;

    fn new(runtime: &'a Runtime, raw_interface: Self::RawInterface) -> Self {
        Self {
            runtime,
            raw_interface,
        }
    }
}

impl<'a> CreatableInterface<'a> for DeviceEnumerator<'a> {
    fn get_guid() -> windows::core::GUID {
        MMDeviceEnumerator
    }
}

impl<'a> DeviceEnumerator<'a> {
    pub(crate) fn get_collection(
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

    pub(crate) fn get(&self, device_id: &String) -> Result<InterfaceWrapper<Device>, Error> {
        let device_id: Vec<_> = device_id.encode_utf16().chain([0]).collect();
        unsafe { self.raw_interface.GetDevice(PCWSTR(device_id.as_ptr())) }
            .map(|raw_interface| self.runtime.wrap_instance(raw_interface))
            .map_err(Error::from)
    }

    pub(crate) fn get_event_stream(&'a self) -> Result<DeviceEventStream<'a>, Error> {
        DeviceEventStream::new(self)
    }

    pub(crate) unsafe fn register_event_client(
        &self,
        event_client: &'a IMMNotificationClient,
    ) -> Result<(), Error> {
        self.raw_interface
            .RegisterEndpointNotificationCallback(event_client)
            .map_err(Error::from)
    }

    pub(crate) unsafe fn unregister_event_client(
        &self,
        event_client: &'a IMMNotificationClient,
    ) -> Result<(), Error> {
        self.raw_interface
            .UnregisterEndpointNotificationCallback(event_client)
            .map_err(Error::from)
    }
}
