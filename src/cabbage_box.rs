use std::ops::{Deref, DerefMut};

use crate::{COLLECTOR, raw::RawCabbage};

#[derive(Debug, Clone)]
pub struct CabbageBox<T> {
    raw_cabbage: RawCabbage,
    _type: std::marker::PhantomData<T>,
}

impl<T> CabbageBox<T> {
    pub fn new(value: T) -> Self {
        let raw_cabbage = RawCabbage::allocate(value);

        // TODO: Collector에 등록

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
