use windows::Win32::System::Com::{CoCreateInstance, CoInitializeEx, CoUninitialize, CLSCTX_ALL};

use crate::error::Error;

use super::{
    creatable_interface::CreatableInterface, interface::Interface,
    interface_wrapper::InterfaceWrapper, runtime_mode::RuntimeMode,
};

pub(crate) struct Runtime {}

impl Runtime {
    pub(crate) fn new(runtime_mode: RuntimeMode) -> Result<Self, Error> {
        unsafe { CoInitializeEx(None, runtime_mode.into()) }
            .map(|_| Runtime {})
            .map_err(Error::Windows)
    }

    pub(crate) fn create<'a, CI: CreatableInterface<'a>>(
        &'a self,
    ) -> Result<InterfaceWrapper<CI>, Error> {
        unsafe { CoCreateInstance(&CI::get_guid(), None, CLSCTX_ALL) }
            .map(|raw_interface| CI::new(self, raw_interface))
            .map(|interface| InterfaceWrapper::new(interface))
            .map_err(Error::Windows)
    }

    pub(crate) fn wrap<'a, I: Interface<'a>>(
        &'a self,
        raw_interface: I::RawInterface,
    ) -> InterfaceWrapper<I> {
        InterfaceWrapper::new(I::new(self, raw_interface))
    }
}

impl Drop for Runtime {
    fn drop(&mut self) {
        unsafe { CoUninitialize() }
    }
}
