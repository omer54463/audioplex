use chrono::{DateTime, Duration, Local, TimeZone, Utc};
use windows::Win32::System::Com::StructuredStorage::{PROPVARIANT, PROPVARIANT_0_0_0};

use crate::error::Error;

use super::property_key::PropertyKey;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum PropertyValue {
    String(String),
    DateTime(DateTime<Local>),
}

impl PropertyValue {
    unsafe fn from_wide_string(propvariant: PROPVARIANT) -> Result<Self, Error> {
        Self::unwrap_propvariant(propvariant)
            .pwszVal
            .to_string()
            .map(Self::String)
            .map_err(Error::from)
    }

    // Microsoft are insane.
    // FILETIME's "0" is January 1, 1601 UTC.
    // FILETIME's interval is 100 nanoseconds (0.1 microseconds).
    // What the actual fuck.

    unsafe fn from_filetime(propvariant: PROPVARIANT) -> Result<Self, Error> {
        let filetime = Self::unwrap_propvariant(propvariant).filetime;
        let file_time_low = filetime.dwLowDateTime;
        let file_time_high = filetime.dwHighDateTime;

        let zero_time = Utc.with_ymd_and_hms(1601, 1, 1, 0, 0, 0).unwrap();
        let intervals_per_microsecond = 10;

        let intervals = (file_time_low as i64) | ((file_time_high as i64) << 32);
        let microseconds = intervals / intervals_per_microsecond;
        let duration = Duration::microseconds(microseconds);
        let datetime = zero_time + duration;

        Ok(Self::DateTime(datetime.into()))
    }

    unsafe fn unwrap_propvariant(propvariant: PROPVARIANT) -> PROPVARIANT_0_0_0 {
        propvariant.Anonymous.Anonymous.Anonymous.clone()
    }
}

impl TryFrom<(PropertyKey, PROPVARIANT)> for PropertyValue {
    type Error = Error;

    fn try_from((property_key, propvariant): (PropertyKey, PROPVARIANT)) -> Result<Self, Error> {
        match property_key {
            PropertyKey::DeviceName => unsafe { Self::from_wide_string(propvariant) },
            PropertyKey::IconPath => unsafe { Self::from_wide_string(propvariant) },
            PropertyKey::DeviceDescription => unsafe { Self::from_wide_string(propvariant) },
            PropertyKey::LastArrivalDate => unsafe { Self::from_filetime(propvariant) },
            PropertyKey::LastRemovalDate => unsafe { Self::from_filetime(propvariant) },
        }
    }
}
