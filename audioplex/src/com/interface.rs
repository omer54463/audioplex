use crate::com::runtime::Runtime;

pub(crate) trait Interface<'a> {
    type UnsafeInterface: ::windows::core::Interface;

    fn new(runtime: &'a Runtime, unsafe_interface: Self::UnsafeInterface) -> Self;
}
