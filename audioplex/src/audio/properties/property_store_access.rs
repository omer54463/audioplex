use crate::error::Error;
use windows::Win32::System::Com::{
    STGM, STGM_CONVERT, STGM_CREATE, STGM_DELETEONRELEASE, STGM_DIRECT, STGM_DIRECT_SWMR,
    STGM_FAILIFTHERE, STGM_NOSCRATCH, STGM_NOSNAPSHOT, STGM_PRIORITY, STGM_READ, STGM_READWRITE,
    STGM_SHARE_DENY_NONE, STGM_SHARE_DENY_READ, STGM_SHARE_DENY_WRITE, STGM_SHARE_EXCLUSIVE,
    STGM_SIMPLE, STGM_TRANSACTED, STGM_WRITE,
};

pub(crate) enum PropertyStoreAccess {
    Direct,
    Transacted,
    Simple,
    Read,
    Write,
    ReadWrite,
    ShareDenyNone,
    ShareDenyRead,
    ShareDenyWrite,
    ShareExclusive,
    Priority,
    DeleteOnRelease,
    NoScratch,
    Create,
    Convert,
    FailIfThere,
    NoSnapshot,
    DirectSwmr,
}

impl TryFrom<STGM> for PropertyStoreAccess {
    type Error = Error;

    fn try_from(access: STGM) -> Result<Self, Self::Error> {
        match access {
            access if access == STGM_DIRECT => Ok(Self::Direct),
            access if access == STGM_TRANSACTED => Ok(Self::Transacted),
            access if access == STGM_SIMPLE => Ok(Self::Simple),
            access if access == STGM_READ => Ok(Self::Read),
            access if access == STGM_WRITE => Ok(Self::Write),
            access if access == STGM_READWRITE => Ok(Self::ReadWrite),
            access if access == STGM_SHARE_DENY_NONE => Ok(Self::ShareDenyNone),
            access if access == STGM_SHARE_DENY_READ => Ok(Self::ShareDenyRead),
            access if access == STGM_SHARE_DENY_WRITE => Ok(Self::ShareDenyWrite),
            access if access == STGM_SHARE_EXCLUSIVE => Ok(Self::ShareExclusive),
            access if access == STGM_PRIORITY => Ok(Self::Priority),
            access if access == STGM_DELETEONRELEASE => Ok(Self::DeleteOnRelease),
            access if access == STGM_NOSCRATCH => Ok(Self::NoScratch),
            access if access == STGM_CREATE => Ok(Self::Create),
            access if access == STGM_CONVERT => Ok(Self::Convert),
            access if access == STGM_FAILIFTHERE => Ok(Self::FailIfThere),
            access if access == STGM_NOSNAPSHOT => Ok(Self::NoSnapshot),
            access if access == STGM_DIRECT_SWMR => Ok(Self::DirectSwmr),
            _ => Err(Error::UnknownPropertyStoreAccess { access }),
        }
    }
}

impl From<PropertyStoreAccess> for STGM {
    fn from(val: PropertyStoreAccess) -> Self {
        match val {
            PropertyStoreAccess::Direct => STGM_DIRECT,
            PropertyStoreAccess::Transacted => STGM_TRANSACTED,
            PropertyStoreAccess::Simple => STGM_SIMPLE,
            PropertyStoreAccess::Read => STGM_READ,
            PropertyStoreAccess::Write => STGM_WRITE,
            PropertyStoreAccess::ReadWrite => STGM_READWRITE,
            PropertyStoreAccess::ShareDenyNone => STGM_SHARE_DENY_NONE,
            PropertyStoreAccess::ShareDenyRead => STGM_SHARE_DENY_READ,
            PropertyStoreAccess::ShareDenyWrite => STGM_SHARE_DENY_WRITE,
            PropertyStoreAccess::ShareExclusive => STGM_SHARE_EXCLUSIVE,
            PropertyStoreAccess::Priority => STGM_PRIORITY,
            PropertyStoreAccess::DeleteOnRelease => STGM_DELETEONRELEASE,
            PropertyStoreAccess::NoScratch => STGM_NOSCRATCH,
            PropertyStoreAccess::Create => STGM_CREATE,
            PropertyStoreAccess::Convert => STGM_CONVERT,
            PropertyStoreAccess::FailIfThere => STGM_FAILIFTHERE,
            PropertyStoreAccess::NoSnapshot => STGM_NOSNAPSHOT,
            PropertyStoreAccess::DirectSwmr => STGM_DIRECT_SWMR,
        }
    }
}
