pub(crate) trait DeviceNotificationCallback {
    fn on_add(&self, _device_id: String) {}
    fn on_remove(&self, _device_id: String) {}
    fn on_state_change(&self, _device_id: String) {}
    fn on_name_change(&self, _device_id: String) {}
    fn on_icon_change(&self, _device_id: String) {}
    fn on_description_change(&self, _device_id: String) {}
}
