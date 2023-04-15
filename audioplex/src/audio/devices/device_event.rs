use crate::com::interface_wrapper::InterfaceWrapper;

use super::{device::Device, device_state::DeviceState};

pub(crate) enum DeviceEvent<'a> {
    Add {
        device_id: String,
        device: InterfaceWrapper<'a, Device<'a>>,
    },
    Remove {
        device_id: String,
    },
    NameChange {
        device_id: String,
        display_name: String,
    },
    IconChange {
        device_id: String,
        icon_path: String,
    },
    DescriptionChange {
        device_id: String,
        description: String,
    },
    StateChange {
        device_id: String,
        device_state: DeviceState,
    },
}
