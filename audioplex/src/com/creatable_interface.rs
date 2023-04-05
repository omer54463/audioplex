use crate::com::interface::Interface;
use windows::core::GUID;

pub(crate) trait CreatableInterface<'a>: Interface<'a>
where
    Self::RawInterface: ::windows::core::Interface,
{
    fn get_guid() -> GUID;
}
