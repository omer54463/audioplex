use crate::audio::properties::property_type::PropertyType;
use crate::error::Error;
use windows::Win32::{
    Devices::{
        FunctionDiscovery::PKEY_Device_FriendlyName,
        Properties::{
            DEVPKEY_DeviceClass_IconPath, DEVPKEY_DeviceInterface_FriendlyName,
            DEVPKEY_Device_DeviceDesc, DEVPKEY_Device_EnumeratorName, DEVPROPKEY,
        },
    },
    UI::Shell::PropertiesSystem::PROPERTYKEY,
};

#[derive(Clone, Copy)]
pub(crate) enum PropertyKey {
    DeviceName,
    IconPath,
    EnumeratorName,
    InterfaceName,
    DeviceDescription,
}

impl PropertyKey {
    pub(crate) fn property_type(&self) -> PropertyType {
        match self {
            Self::DeviceName => PropertyType::String,
            Self::IconPath => PropertyType::String,
            Self::EnumeratorName => PropertyType::String,
            Self::InterfaceName => PropertyType::String,
            Self::DeviceDescription => PropertyType::String,
        }
    }
}

impl TryFrom<PROPERTYKEY> for PropertyKey {
    type Error = Error;

    fn try_from(property_key: PROPERTYKEY) -> Result<Self, Self::Error> {
        match property_key.fmtid {
            fmtid if fmtid == PKEY_Device_FriendlyName.fmtid => Ok(Self::DeviceName),
            fmtid if fmtid == DEVPKEY_DeviceClass_IconPath.fmtid => Ok(Self::IconPath),
            fmtid if fmtid == DEVPKEY_Device_EnumeratorName.fmtid => Ok(Self::EnumeratorName),
            fmtid if fmtid == DEVPKEY_DeviceInterface_FriendlyName.fmtid => Ok(Self::InterfaceName),
            fmtid if fmtid == DEVPKEY_Device_DeviceDesc.fmtid => Ok(Self::DeviceDescription),
            _ => Err(Error::UnknownPropertyKey { property_key }),
        }
    }
}

impl From<PropertyKey> for PROPERTYKEY {
    fn from(val: PropertyKey) -> Self {
        match val {
            PropertyKey::DeviceName => PKEY_Device_FriendlyName,
            PropertyKey::IconPath => to_property_key(DEVPKEY_DeviceClass_IconPath),
            PropertyKey::EnumeratorName => to_property_key(DEVPKEY_Device_EnumeratorName),
            PropertyKey::InterfaceName => to_property_key(DEVPKEY_DeviceInterface_FriendlyName),
            PropertyKey::DeviceDescription => to_property_key(DEVPKEY_Device_DeviceDesc),
        }
    }
}

fn to_property_key(device_property_key: DEVPROPKEY) -> PROPERTYKEY {
    PROPERTYKEY {
        fmtid: device_property_key.fmtid,
        pid: device_property_key.pid,
    }
}
