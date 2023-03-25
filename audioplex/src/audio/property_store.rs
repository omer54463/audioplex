use crate::com::{interface::Interface, runtime::Runtime};
use crate::{audio::property_key::PropertyKey, audio::property_type::PropertyType, error::Error};
use windows::Win32::{
    System::Com::StructuredStorage::PROPVARIANT, UI::Shell::PropertiesSystem::IPropertyStore,
};

pub(crate) struct PropertyStore<'a> {
    runtime: &'a Runtime,
    unsafe_interface: IPropertyStore,
}

impl<'a> Interface<'a> for PropertyStore<'a> {
    type UnsafeInterface = IPropertyStore;

    fn new(runtime: &'a Runtime, unsafe_interface: Self::UnsafeInterface) -> Self {
        Self {
            runtime,
            unsafe_interface,
        }
    }
}

impl<'a> PropertyStore<'a> {
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
        self.unsafe_interface
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
