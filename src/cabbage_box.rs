use core::fmt;
use std::ops::{Deref, DerefMut};

use crate::{COLLECTOR, raw::RawCabbage};

#[derive(Clone)]
pub struct CabbageBox<T> {
    pub(crate) raw_cabbage: RawCabbage,
    pub(crate) _type: std::marker::PhantomData<T>,
}

impl<T> CabbageBox<T> {
    pub fn new(value: T) -> Self {
        let raw_cabbage = RawCabbage::allocate(value);

        COLLECTOR.allocate_to_roots(raw_cabbage.clone());

        CabbageBox {
            raw_cabbage,
            _type: std::marker::PhantomData,
        }
    }

    fn get_data_ref(&self) -> &T {
        self.raw_cabbage.get_data_ref()
    }

    fn get_data_mut(&mut self) -> &mut T {
        self.raw_cabbage.get_data_mut()
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for CabbageBox<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt::Debug::fmt(self.get_data_ref(), f)
    }
}

impl<T> Deref for CabbageBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get_data_ref()
    }
}

impl<T> DerefMut for CabbageBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_data_mut()
    }
}
