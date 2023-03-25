#![allow(dead_code)]

mod audio;
mod com;
mod error;

use crate::audio::property_key::PropertyKey;
use crate::audio::property_store_access::PropertyStoreAccess;
use crate::com::{runtime::Runtime, runtime_mode::RuntimeMode};
use audio::data_flow::DataFlow;
use audio::device_enumerator::DeviceEnumerator;
use audio::device_state::DeviceState;
use error::Error;
use windows::Win32::Media::Audio::MMDeviceEnumerator;

fn main() -> Result<(), Error> {
    let runtime = Runtime::new(RuntimeMode::MultiThreaded)?;

    let device_enumerator = runtime.create_instance::<DeviceEnumerator>(&MMDeviceEnumerator)?;

    let device_collection =
        device_enumerator.get_device_collection(DataFlow::All, DeviceState::Active)?;

    for device_index in 0..device_collection.get_count()? {
        println!("---------------------------------------------------------------------------");

        let device = device_collection.get_device(device_index)?;

        let device_id = device.get_id()?;
        println!("Device ID: {}", device_id);

        let property_store = device.get_property_store(PropertyStoreAccess::Read)?;

        let device_name = property_store.get_string(PropertyKey::DeviceName)?;
        println!("Device Name: {}", device_name);

        let enumerator_name = property_store.get_string(PropertyKey::EnumeratorName)?;
        println!("Enumerator Name: {}", enumerator_name);

        let interface_name = property_store.get_string(PropertyKey::InterfaceName)?;
        println!("Interface Name: {}", interface_name);

        let device_description = property_store.get_string(PropertyKey::DeviceDescription)?;
        println!("Device Description: {}", device_description);
    }

    println!("---------------------------------------------------------------------------");

    Ok(())
}
