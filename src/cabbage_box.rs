use core::fmt;
use std::ops::{Deref, DerefMut};

use crate::{COLLECTOR, raw::RawCabbage};

/// Object Wrapper managed by the Cabbage Collector
/// This can share same object

pub struct CabbageBox<T> {
    pub raw_cabbage: *mut RawCabbage,
    pub(crate) _type: std::marker::PhantomData<T>,
}

impl<T> CabbageBox<T> {
    pub fn new_root(value: T) -> Self {
        let raw_cabbage = RawCabbage::allocate(value);
        unsafe {
            (*raw_cabbage).is_root = true;
        }

        COLLECTOR.roots.borrow_mut().push(raw_cabbage);
        COLLECTOR.all_objects.borrow_mut().push(raw_cabbage);

        CabbageBox {
            raw_cabbage,
            _type: std::marker::PhantomData,
        }
    }

    pub fn new_non_root(value: T) -> CabbageBox<T> {
        let raw_cabbage = RawCabbage::allocate(value);
        unsafe {
            (*raw_cabbage).is_root = false;
        }

        COLLECTOR.all_objects.borrow_mut().push(raw_cabbage);

        CabbageBox {
            raw_cabbage,
            _type: std::marker::PhantomData,
        }
    }

    pub fn adopt_child<U>(&mut self, child: CabbageBox<U>) {
        unsafe {
            (*self.raw_cabbage).child_objects.push(child.raw_cabbage);
        }
    }

    fn get_data_ref(&self) -> &T {
        unsafe { (*self.raw_cabbage).get_data_ref() }
    }

    fn get_data_mut(&mut self) -> &mut T {
        unsafe { (*self.raw_cabbage).get_data_mut_ref() }
    }
}

impl<T> Clone for CabbageBox<T> {
    fn clone(&self) -> Self {
        unsafe {
            if (*self.raw_cabbage).is_root {
                COLLECTOR.roots.borrow_mut().push(self.raw_cabbage);
            }
        }

        CabbageBox {
            raw_cabbage: self.raw_cabbage,
            _type: std::marker::PhantomData,
        }
    }
}

impl<T> Drop for CabbageBox<T> {
    fn drop(&mut self) {
        unsafe {
            // roots 객체 목록에서 제거
            if (*self.raw_cabbage).is_root {
                let mut roots = COLLECTOR.roots.borrow_mut();

                // 포인터 값이 일치하는 것 하나만 제거
                let mut index = -1;

                for (i, obj) in roots.iter().enumerate() {
                    if (**obj).data_ptr == (*self.raw_cabbage).data_ptr {
                        index = i as isize;
                        break;
                    }
                }

                if index >= 0 {
                    roots.remove(index as usize);
                }
            }
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
