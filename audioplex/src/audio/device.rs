use crate::com::{interface::Interface, interface_wrapper::InterfaceWrapper, runtime::Runtime};
use crate::{
    audio::{property_store::PropertyStore, property_store_access::PropertyStoreAccess},
    error::Error,
};
use windows::Win32::Media::Audio::IMMDevice;

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
}
