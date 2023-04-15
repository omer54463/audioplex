use windows::Win32::{
    Devices::{
        FunctionDiscovery::PKEY_Device_FriendlyName,
        Properties::{
            DEVPKEY_DeviceClass_IconPath, DEVPKEY_Device_DeviceDesc,
            DEVPKEY_Device_LastArrivalDate, DEVPKEY_Device_LastRemovalDate, DEVPROPKEY,
        },
    },
    UI::Shell::PropertiesSystem::PROPERTYKEY,
};

use crate::error::Error;

#[derive(Debug, Clone, Copy)]
pub(crate) enum PropertyKey {
    DeviceName,
    IconPath,
    DeviceDescription,
    LastArrivalDate,
    LastRemovalDate,
}

impl TryFrom<PROPERTYKEY> for PropertyKey {
    type Error = Error;

    fn try_from(property_key: PROPERTYKEY) -> Result<Self, Self::Error> {
        match property_key.fmtid {
            fmtid if fmtid == PKEY_Device_FriendlyName.fmtid => Ok(Self::DeviceName),
            fmtid if fmtid == DEVPKEY_DeviceClass_IconPath.fmtid => Ok(Self::IconPath),
            fmtid if fmtid == DEVPKEY_Device_DeviceDesc.fmtid => Ok(Self::DeviceDescription),
            fmtid if fmtid == DEVPKEY_Device_LastArrivalDate.fmtid => Ok(Self::LastArrivalDate),
            fmtid if fmtid == DEVPKEY_Device_LastRemovalDate.fmtid => Ok(Self::LastRemovalDate),
            _ => Err(Error::UnknownPropertyKey { property_key }),
        }
    }
}

impl From<PropertyKey> for PROPERTYKEY {
    fn from(val: PropertyKey) -> Self {
        match val {
            PropertyKey::DeviceName => PKEY_Device_FriendlyName,
            PropertyKey::IconPath => to_property_key(DEVPKEY_DeviceClass_IconPath),
            PropertyKey::DeviceDescription => to_property_key(DEVPKEY_Device_DeviceDesc),
            PropertyKey::LastArrivalDate => to_property_key(DEVPKEY_Device_LastArrivalDate),
            PropertyKey::LastRemovalDate => to_property_key(DEVPKEY_Device_LastRemovalDate),
        }
    }
}

fn to_property_key(device_property_key: DEVPROPKEY) -> PROPERTYKEY {
    PROPERTYKEY {
        fmtid: device_property_key.fmtid,
        pid: device_property_key.pid,
    }
}
