use crate::{
    com::{interface::Interface, interface_wrapper::InterfaceWrapper, runtime_mode::RuntimeMode},
    error::Error,
};
use windows::{
    core::GUID,
    Win32::System::Com::{CoCreateInstance, CoInitializeEx, CoUninitialize, CLSCTX_ALL},
};

pub(crate) struct Runtime {}

impl Runtime {
    pub(crate) fn new(runtime_mode: RuntimeMode) -> Result<Self, Error> {
        unsafe { CoInitializeEx(None, runtime_mode.into()) }
            .map(|_| Runtime {})
            .map_err(|error| Error::WindowsError(error))
    }

    pub(crate) fn create_instance<'a, I: Interface<'a>>(
        &'a self,
        guid: &GUID,
    ) -> Result<InterfaceWrapper<I>, Error> {
        unsafe { CoCreateInstance(guid, None, CLSCTX_ALL) }
            .map(|unsafe_interface| I::new(&self, unsafe_interface))
            .map(|interface| InterfaceWrapper::new(interface))
            .map_err(|error| Error::WindowsError(error))
    }

    pub(crate) fn wrap_instance<'a, I: Interface<'a>>(
        &'a self,
        unsafe_interface: I::UnsafeInterface,
    ) -> InterfaceWrapper<I> {
        InterfaceWrapper::new(I::new(&self, unsafe_interface))
    }
}

impl Drop for Runtime {
    fn drop(&mut self) {
        unsafe { CoUninitialize() }
    }
}
