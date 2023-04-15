use windows::core::GUID;

use super::interface::Interface;

pub(crate) trait CreatableInterface<'a>: Interface<'a> {
    fn get_guid() -> GUID;
}
