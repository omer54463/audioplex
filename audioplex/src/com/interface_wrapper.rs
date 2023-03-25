use crate::com::{interface::Interface, runtime::Runtime};
use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

pub(crate) struct InterfaceWrapper<'a, I: Interface<'a>> {
    interface: I,
    marker: PhantomData<&'a Runtime>,
}

impl<'a, I: Interface<'a>> InterfaceWrapper<'a, I> {
    pub(crate) fn new(interface: I) -> Self {
        Self {
            interface,
            marker: PhantomData::default(),
        }
    }
}

impl<'a, I: Interface<'a>> Deref for InterfaceWrapper<'a, I> {
    type Target = I;

    fn deref(&self) -> &Self::Target {
        &self.interface
    }
}

impl<'a, I: Interface<'a>> DerefMut for InterfaceWrapper<'a, I> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.interface
    }
}
