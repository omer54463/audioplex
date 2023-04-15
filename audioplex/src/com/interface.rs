use super::runtime::Runtime;

pub(crate) trait Interface<'a> {
    type RawInterface: ::windows::core::Interface;

    fn new(runtime: &'a Runtime, raw_interface: Self::RawInterface) -> Self;
}
