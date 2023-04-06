#![feature(if_let_guard)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]

mod audio;
mod com;
mod error;

use crate::audio::data_flow::DataFlow;
use crate::audio::devices::device_enumerator::DeviceEnumerator;
use crate::audio::devices::device_state::DeviceState;
use crate::audio::properties::property_key::PropertyKey;
use crate::audio::properties::property_store_access::PropertyStoreAccess;
use crate::com::{runtime::Runtime, runtime_mode::RuntimeMode};
use crate::error::Error;

fn main() -> Result<(), Error> {
    let runtime = Runtime::new(RuntimeMode::SingleThreaded)?;

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

            let icon_path = property_store.get_string(PropertyKey::IconPath)?;
            println!("Icon Path: {}", icon_path);

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

                let display_name = session_control.get_display_name()?;
                println!("- Display Name: {}", display_name);

                let icon_path = session_control.get_icon_path()?;
                println!("- Icon Path: {}", icon_path);

                let session_state = session_control.get_state()?;
                println!("- Session State: {:?}", session_state);

                let session_extended_control = session_control.get_extended_control()?;

                let is_system = session_extended_control.is_system()?;
                println!("- Is System: {}", is_system);

                let process_id = session_extended_control.get_process_id()?;
                println!("- Process ID: {:?}", process_id);
            }
        }
    }

    println!("---------------------------------------------------------------------------");

    let device_event_stream = device_enumerator.get_device_event_stream()?;

    loop {
        match device_event_stream.recv() {
            Ok(device_event) => println!("{:?}", device_event),
            Err(_) => break,
        }
    }

    Ok(())
}
