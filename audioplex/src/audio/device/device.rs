use crate::audio::device::device_state::DeviceState;
use crate::audio::session::session_manager::SessionManager;
use crate::com::{interface::Interface, interface_wrapper::InterfaceWrapper, runtime::Runtime};
use crate::{
    audio::property::{property_store::PropertyStore, property_store_access::PropertyStoreAccess},
    error::Error,
};
use windows::Win32::Media::Audio::IMMDevice;
use windows::Win32::System::Com::CLSCTX_ALL;

pub(crate) struct Device<'a> {
    runtime: &'a Runtime,
    unsafe_interface: IMMDevice,
}

impl<'a> Interface<'a> for Device<'a> {
    type UnsafeInterface = IMMDevice;

    fn new(runtime: &'a Runtime, unsafe_interface: Self::UnsafeInterface) -> Self {
        Self {
            runtime,
            unsafe_interface,
        }
    }
}

impl<'a> Device<'a> {
    pub(crate) fn get_id(&self) -> Result<String, Error> {
        unsafe { self.unsafe_interface.GetId() }
            .map_err(Error::from)
            .and_then(|id| unsafe { id.to_string() }.map_err(Error::from))
    }

    pub(crate) fn get_state(&self) -> Result<DeviceState, Error> {
        unsafe { self.unsafe_interface.GetState() }
            .map_err(Error::from)
            .and_then(|state| state.try_into())
    }

    pub(crate) fn get_property_store(
        &self,
        property_access: PropertyStoreAccess,
    ) -> Result<InterfaceWrapper<PropertyStore>, Error> {
        unsafe {
            self.unsafe_interface
                .OpenPropertyStore(property_access.into())
        }
        .map(|unsafe_interface| self.runtime.wrap_instance(unsafe_interface))
        .map_err(Error::from)
    }

    pub(crate) fn get_session_manager(
        &self,
    ) -> Result<InterfaceWrapper<'a, SessionManager<'a>>, Error> {
        unsafe { self.unsafe_interface.Activate(CLSCTX_ALL, None) }
            .map(|unsafe_interface| self.runtime.wrap_instance(unsafe_interface))
            .map_err(Error::from)
    }
}
