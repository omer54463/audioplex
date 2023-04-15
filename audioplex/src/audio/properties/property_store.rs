use windows::Win32::UI::Shell::PropertiesSystem::IPropertyStore;

use crate::{
    com::{interface::Interface, runtime::Runtime},
    error::Error,
};

use super::{property_key::PropertyKey, property_value::PropertyValue};

pub(crate) struct PropertyStore {
    raw_interface: IPropertyStore,
}

impl<'a> Interface<'a> for PropertyStore {
    type RawInterface = IPropertyStore;

    fn new(_runtime: &'a Runtime, raw_interface: Self::RawInterface) -> Self {
        Self { raw_interface }
    }
}

impl PropertyStore {
    pub(crate) fn get_string(&self, property_key: PropertyKey) -> Result<String, Error> {
        let property_value = self.get(property_key)?;

        match property_value {
            PropertyValue::String(string) => Ok(string),
            _ => Err(Error::UnexpectedPropertyType { property_key }),
        }
    }

    fn get(&self, property_key: PropertyKey) -> Result<PropertyValue, Error> {
        unsafe { self.raw_interface.GetValue(&property_key.into()) }
            .map_err(Error::from)
            .and_then(|propvariant| (property_key, propvariant).try_into())
    }
}
