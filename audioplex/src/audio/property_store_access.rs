use windows::Win32::System::Com::{
    STGM, STGM_CONVERT, STGM_CREATE, STGM_DELETEONRELEASE, STGM_DIRECT, STGM_DIRECT_SWMR,
    STGM_FAILIFTHERE, STGM_NOSCRATCH, STGM_NOSNAPSHOT, STGM_PRIORITY, STGM_READ, STGM_READWRITE,
    STGM_SHARE_DENY_NONE, STGM_SHARE_DENY_READ, STGM_SHARE_DENY_WRITE, STGM_SHARE_EXCLUSIVE,
    STGM_SIMPLE, STGM_TRANSACTED, STGM_WRITE,
};

use crate::error::Error;

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

impl Into<STGM> for PropertyStoreAccess {
    fn into(self) -> STGM {
        match self {
            Self::Direct => STGM_DIRECT,
            Self::Transacted => STGM_TRANSACTED,
            Self::Simple => STGM_SIMPLE,
            Self::Read => STGM_READ,
            Self::Write => STGM_WRITE,
            Self::ReadWrite => STGM_READWRITE,
            Self::ShareDenyNone => STGM_SHARE_DENY_NONE,
            Self::ShareDenyRead => STGM_SHARE_DENY_READ,
            Self::ShareDenyWrite => STGM_SHARE_DENY_WRITE,
            Self::ShareExclusive => STGM_SHARE_EXCLUSIVE,
            Self::Priority => STGM_PRIORITY,
            Self::DeleteOnRelease => STGM_DELETEONRELEASE,
            Self::NoScratch => STGM_NOSCRATCH,
            Self::Create => STGM_CREATE,
            Self::Convert => STGM_CONVERT,
            Self::FailIfThere => STGM_FAILIFTHERE,
            Self::NoSnapshot => STGM_NOSNAPSHOT,
            Self::DirectSwmr => STGM_DIRECT_SWMR,
        }
    }
}
