#![feature(if_let_guard)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]

mod audio;
mod com;
mod error;

use crate::audio::devices::device_enumerator::DeviceEnumerator;
use crate::audio::sessions::session_manager_event::SessionManagerEvent;
use crate::com::{runtime::Runtime, runtime_mode::RuntimeMode};
use crate::error::Error;

fn main() -> Result<(), Error> {
    let runtime = Runtime::new(RuntimeMode::SingleThreaded)?;

    let device_enumerator = runtime.create_instance::<DeviceEnumerator>()?;

    let device = device_enumerator.get(String::from(
        "{0.0.0.00000000}.{61e87334-029c-40b3-93ab-69ead02d5cd1}",
    ))?;

    let session_manager = device.get_session_manager()?;

    let event_stream = session_manager.get_event_stream()?;

    loop {
        match event_stream.recv() {
            Ok(SessionManagerEvent::Add { session }) => println!("{:?}", session.get_process_id()?),
            Err(_) => break,
        }
    }

    Ok(())
}
