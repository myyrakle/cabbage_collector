use core::fmt;
use std::ops::{Deref, DerefMut};

use crate::{COLLECTOR, raw::RawCabbage};

#[derive(Clone)]
pub struct CabbageBox<T> {
    pub(crate) raw_cabbage: RawCabbage,
    pub(crate) _type: std::marker::PhantomData<T>,
    pub(crate) is_root: bool,
}

impl<T> CabbageBox<T> {
    pub fn new(value: T) -> Self {
        let raw_cabbage = RawCabbage::allocate(value);

        COLLECTOR.allocate_to_roots(raw_cabbage.clone());

        CabbageBox {
            raw_cabbage,
            _type: std::marker::PhantomData,
            is_root: true,
        }
    }

    fn get_data_ref(&self) -> &T {
        self.raw_cabbage.get_data_ref()
    }

    fn get_data_mut(&mut self) -> &mut T {
        self.raw_cabbage.get_data_mut()
    }
}

impl<T> Drop for CabbageBox<T> {
    fn drop(&mut self) {
        // roots 객체 목록에서 제거
        if self.is_root {
            println!("root에서 제거");

            COLLECTOR
                .roots
                .borrow_mut()
                .retain(|obj| obj.borrow().data_ptr != self.raw_cabbage.data_ptr);
        }
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
