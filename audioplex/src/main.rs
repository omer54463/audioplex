#![feature(if_let_guard)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]

use audio::{devices::device_enumerator::DeviceEnumerator, sessions::session_event::SessionEvent};
use com::{runtime::Runtime, runtime_mode::RuntimeMode};
use error::Error;

mod audio;
mod com;
mod error;

fn main() -> Result<(), Error> {
    let runtime = Runtime::new(RuntimeMode::MultiThreaded)?;

    let device_enumerator = runtime.create::<DeviceEnumerator>()?;

    let device = device_enumerator.get(&String::from(
        "{0.0.0.00000000}.{61e87334-029c-40b3-93ab-69ead02d5cd1}",
    ))?;

    let session_manager = device.get_session_manager()?;

    let event_stream = session_manager.get_event_stream()?;

    loop {
        match event_stream.recv() {
            Ok(SessionEvent::Add {
                session_id,
                session,
            }) => println!("{} {}", session_id, session.get_process_id()?),
            Ok(_) => {}
            Err(_) => break,
        }
    }

    Ok(())
}
