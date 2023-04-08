use crate::{audio::devices::device::Device, com::interface_wrapper::InterfaceWrapper};

pub(crate) enum DeviceEvent<'a> {
    Add {
        device: InterfaceWrapper<'a, Device<'a>>,
    },
    Remove {
        device: InterfaceWrapper<'a, Device<'a>>,
    },
    NameChange {
        device: InterfaceWrapper<'a, Device<'a>>,
    },
    IconChange {
        device: InterfaceWrapper<'a, Device<'a>>,
    },
    DescriptionChange {
        device: InterfaceWrapper<'a, Device<'a>>,
    },
    StateChange {
        device: InterfaceWrapper<'a, Device<'a>>,
    },
}
