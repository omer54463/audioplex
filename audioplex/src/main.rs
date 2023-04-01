#![feature(if_let_guard)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]

mod audio;
mod com;
mod error;

use crate::audio::property::property_key::PropertyKey;
use crate::audio::property::property_store_access::PropertyStoreAccess;
use crate::com::{runtime::Runtime, runtime_mode::RuntimeMode};
use audio::data_flow::DataFlow;
use audio::device::device_enumerator::DeviceEnumerator;
use audio::device::device_state::DeviceState;
use error::Error;

fn main() -> Result<(), Error> {
    let runtime = Runtime::new(RuntimeMode::MultiThreaded)?;

    let device_enumerator = runtime.create_instance::<DeviceEnumerator>()?;

    let device_collection =
        device_enumerator.get_device_collection(DataFlow::All, DeviceState::All)?;

    for device_index in 0..device_collection.get_count()? {
        println!("---------------------------------------------------------------------------");

        let device = device_collection.get_device(device_index)?;

        let device_id = device.get_id()?;
        println!("Device ID: {}", device_id);

        let device_state = device.get_state()?;
        println!("Device State: {:?}", device_state);

        if device_state != DeviceState::NotPresent {
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

        if device_state == DeviceState::Active {
            let session_manager = device.get_session_manager()?;

            let session_enumerator = session_manager.get_session_enumerator()?;

            let session_count = session_enumerator.get_count()?;
            println!("Session Count: {}", session_count);

            for session_index in 0..session_count {
                let session_control = session_enumerator.get_session_control(session_index)?;

                let session_display_name = session_control.get_display_name()?;
                println!("- Session Display Name: {}", session_display_name);

                let session_state = session_control.get_state()?;
                println!("- Session State: {:?}", session_state);

                let session_extended_control = session_control.get_extended_control()?;

                let session_is_system_sounds = session_extended_control.is_system_sounds()?;
                println!("- Is System Sounds: {}", session_is_system_sounds);

                let process_id = session_extended_control.get_process_id()?;
                println!("- Process ID: {:?}", process_id);
            }
        }
    }

    println!("---------------------------------------------------------------------------");

    Ok(())
}
