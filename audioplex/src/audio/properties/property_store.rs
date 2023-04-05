use crate::com::{interface::Interface, runtime::Runtime};
use crate::{
    audio::properties::property_key::PropertyKey, audio::properties::property_type::PropertyType,
    error::Error,
};
use windows::Win32::{
    System::Com::StructuredStorage::PROPVARIANT, UI::Shell::PropertiesSystem::IPropertyStore,
};

pub(crate) struct PropertyStore {
    raw_interface: IPropertyStore,
}

impl<'a> Interface<'a> for PropertyStore {
    type RawInterface = IPropertyStore;

    fn new(_: &'a Runtime, raw_interface: Self::RawInterface) -> Self {
        Self { raw_interface }
    }
}

impl PropertyStore {
    pub(crate) fn get_string(&self, property_key: PropertyKey) -> Result<String, Error> {
        if property_key.property_type() == PropertyType::String {
            unsafe {
                self.get_property_variant(property_key)
                    .and_then(|property_variant| Self::parse_string(property_variant))
            }
        } else {
            Err(Error::UnexpectedPropertyType {
                expected_type: PropertyType::String,
                found_type: property_key.property_type(),
            })
        }
    }

    unsafe fn get_property_variant(&self, property_key: PropertyKey) -> Result<PROPVARIANT, Error> {
        self.raw_interface
            .GetValue(&property_key.into())
            .map_err(Error::from)
    }

    unsafe fn parse_string(propvariant: PROPVARIANT) -> Result<String, Error> {
        propvariant
            .Anonymous
            .Anonymous
            .Anonymous
            .pwszVal
            .to_string()
            .map_err(Error::from)
    }
}
