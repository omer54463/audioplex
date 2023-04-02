use crate::error::Error;
use windows::Win32::Media::Audio::{eAll, eCapture, eRender, EDataFlow};

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

impl From<DataFlow> for EDataFlow {
    fn from(val: DataFlow) -> Self {
        match val {
            DataFlow::Render => eRender,
            DataFlow::Capture => eCapture,
            DataFlow::All => eAll,
        }
    }
}
