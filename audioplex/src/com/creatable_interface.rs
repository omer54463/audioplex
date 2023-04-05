use crate::com::interface::Interface;
use windows::core::GUID;

pub(crate) trait CreatableInterface<'a>: Interface<'a> {
    fn get_guid() -> GUID;
}
