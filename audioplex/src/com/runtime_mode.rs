use windows::Win32::System::Com::{COINIT, COINIT_APARTMENTTHREADED, COINIT_MULTITHREADED};

use crate::error::Error;

pub(crate) enum RuntimeMode {
    SingleThreaded,
    MultiThreaded,
}

impl TryFrom<COINIT> for RuntimeMode {
    type Error = Error;

    fn try_from(runtime_mode: COINIT) -> Result<Self, Self::Error> {
        match runtime_mode {
            runtime_mode if runtime_mode == COINIT_APARTMENTTHREADED => Ok(Self::SingleThreaded),
            runtime_mode if runtime_mode == COINIT_MULTITHREADED => Ok(Self::MultiThreaded),
            _ => Err(Error::UnknownRuntimeMode { runtime_mode }),
        }
    }
}

impl Into<COINIT> for RuntimeMode {
    fn into(self) -> COINIT {
        match self {
            Self::SingleThreaded => COINIT_APARTMENTTHREADED,
            Self::MultiThreaded => COINIT_MULTITHREADED,
        }
    }
}
