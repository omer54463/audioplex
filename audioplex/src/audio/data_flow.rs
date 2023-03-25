use windows::Win32::Media::Audio::{eAll, eCapture, eRender, EDataFlow};

use crate::error::Error;

pub(crate) enum DataFlow {
    Render,
    Capture,
    All,
}

impl TryFrom<EDataFlow> for DataFlow {
    type Error = Error;

    fn try_from(data_flow: EDataFlow) -> Result<Self, Self::Error> {
        match data_flow {
            data_flow if data_flow == eRender => Ok(Self::Render),
            data_flow if data_flow == eCapture => Ok(Self::Capture),
            data_flow if data_flow == eAll => Ok(Self::All),
            _ => Err(Error::UnknownDataFlow { data_flow }),
        }
    }
}

impl Into<EDataFlow> for DataFlow {
    fn into(self) -> EDataFlow {
        match self {
            Self::Render => eRender,
            Self::Capture => eCapture,
            Self::All => eAll,
        }
    }
}
