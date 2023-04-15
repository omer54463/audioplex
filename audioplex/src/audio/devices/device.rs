use crate::audio::devices::device_state::DeviceState;
use crate::audio::sessions::session_manager::SessionManager;
use crate::com::{interface::Interface, interface_wrapper::InterfaceWrapper, runtime::Runtime};
use crate::{
    audio::properties::{
        property_store::PropertyStore, property_store_access::PropertyStoreAccess,
    },
    error::Error,
};
use windows::Win32::Media::Audio::IMMDevice;
use windows::Win32::System::Com::CLSCTX_ALL;

pub(crate) struct Device<'a> {
    runtime: &'a Runtime,
    raw_interface: IMMDevice,
}

impl<'a> Interface<'a> for Device<'a> {
    type RawInterface = IMMDevice;

    fn new(runtime: &'a Runtime, raw_interface: Self::RawInterface) -> Self {
        Self {
            runtime,
            raw_interface,
        }
    }
}

impl<'a> Device<'a> {
    pub(crate) fn get_id(&self) -> Result<String, Error> {
        unsafe { self.raw_interface.GetId() }
            .map_err(Error::from)
            .and_then(|id| unsafe { id.to_string() }.map_err(Error::from))
    }

    pub(crate) fn get_state(&self) -> Result<DeviceState, Error> {
        unsafe { self.raw_interface.GetState() }
            .map_err(Error::from)
            .and_then(|state| state.try_into())
    }

    pub(crate) fn get_property_store(
        &self,
        property_access: PropertyStoreAccess,
    ) -> Result<InterfaceWrapper<PropertyStore>, Error> {
        unsafe { self.raw_interface.OpenPropertyStore(property_access.into()) }
            .map(|raw_interface| self.runtime.wrap(raw_interface))
            .map_err(Error::from)
    }

    pub(crate) fn get_session_manager(&self) -> Result<InterfaceWrapper<SessionManager>, Error> {
        unsafe { self.raw_interface.Activate(CLSCTX_ALL, None) }
            .map(|raw_interface| self.runtime.wrap(raw_interface))
            .map_err(Error::from)
    }
}
