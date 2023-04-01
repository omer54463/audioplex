use crate::audio::property::property_type::PropertyType;
use crate::error::Error;
use windows::Win32::{
    Devices::{
        FunctionDiscovery::PKEY_Device_FriendlyName,
        Properties::{
            DEVPKEY_DeviceInterface_FriendlyName, DEVPKEY_Device_DeviceDesc,
            DEVPKEY_Device_EnumeratorName, DEVPROPKEY,
        },
    },
    UI::Shell::PropertiesSystem::PROPERTYKEY,
};

#[derive(Clone, Copy)]
pub(crate) enum PropertyKey {
    DeviceName,
    EnumeratorName,
    InterfaceName,
    DeviceDescription,
}

impl PropertyKey {
    pub(crate) fn property_type(&self) -> PropertyType {
        match self {
            Self::DeviceName => PropertyType::String,
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
            fmtid if fmtid == DEVPKEY_Device_EnumeratorName.fmtid => Ok(Self::EnumeratorName),
            fmtid if fmtid == DEVPKEY_DeviceInterface_FriendlyName.fmtid => Ok(Self::InterfaceName),
            fmtid if fmtid == DEVPKEY_Device_DeviceDesc.fmtid => Ok(Self::DeviceDescription),
            _ => Err(Error::UnknownPropertyKey { property_key }),
        }
    }
}

impl Into<PROPERTYKEY> for PropertyKey {
    fn into(self) -> PROPERTYKEY {
        match self {
            Self::DeviceName => PKEY_Device_FriendlyName,
            Self::EnumeratorName => to_property_key(DEVPKEY_Device_EnumeratorName),
            Self::InterfaceName => to_property_key(DEVPKEY_DeviceInterface_FriendlyName),
            Self::DeviceDescription => to_property_key(DEVPKEY_Device_DeviceDesc),
        }
    }
}

fn to_property_key(device_property_key: DEVPROPKEY) -> PROPERTYKEY {
    return PROPERTYKEY {
        fmtid: device_property_key.fmtid,
        pid: device_property_key.pid,
    };
}
