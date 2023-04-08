use crate::audio::sessions::session::Session;
use crate::com::interface_wrapper::InterfaceWrapper;

pub(crate) enum SessionManagerEvent<'a> {
    Add {
        session: InterfaceWrapper<'a, Session>,
    },
}
