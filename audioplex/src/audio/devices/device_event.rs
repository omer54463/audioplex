#[derive(Debug)]
pub(crate) enum DeviceEvent {
    Add { device_id: String },
    Remove { device_id: String },
    NameChange { device_id: String },
    IconChange { device_id: String },
    DescriptionChange { device_id: String },
    StateChange { device_id: String },
}
