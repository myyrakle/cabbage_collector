use core::fmt;
use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use crate::{COLLECTOR, raw::RawCabbage};

/// Object Wrapper managed by the Cabbage Collector
/// This can share same object

pub struct CabbageBox<T> {
    pub(crate) raw_cabbage: RawCabbage,
    pub(crate) _type: std::marker::PhantomData<T>,
    pub(crate) is_root: bool,
}

impl<T> CabbageBox<T> {
    pub fn new_root(value: T) -> Self {
        let raw_cabbage = RawCabbage::allocate(value);

        {
            let raw_cabbage = Rc::new(RefCell::new(raw_cabbage.clone()));

            COLLECTOR.roots.borrow_mut().push(raw_cabbage.clone());
            COLLECTOR.all_objects.borrow_mut().push(raw_cabbage.clone());
        }

        CabbageBox {
            raw_cabbage,
            _type: std::marker::PhantomData,
            is_root: true,
        }
    }

    pub fn new_non_root(value: T) -> CabbageBox<T> {
        let raw_cabbage = RawCabbage::allocate(value);

        {
            let raw_cabbage = Rc::new(RefCell::new(raw_cabbage.clone()));

            COLLECTOR.all_objects.borrow_mut().push(raw_cabbage.clone());
        }

        CabbageBox {
            raw_cabbage,
            _type: std::marker::PhantomData,
            is_root: false,
        }
    }

    pub fn adopt_child<U>(&mut self, child: CabbageBox<U>) {
        let raw_cabbage = Rc::new(RefCell::new(child.raw_cabbage.clone()));

        self.raw_cabbage
            .child_objects
            .push(Rc::downgrade(&raw_cabbage));
    }

    fn get_data_ref(&self) -> &T {
        self.raw_cabbage.get_data_ref()
    }

    fn get_data_mut(&mut self) -> &mut T {
        self.raw_cabbage.get_data_mut()
    }
}

impl<T> Clone for CabbageBox<T> {
    fn clone(&self) -> Self {
        let raw_cabbage = Rc::new(RefCell::new(self.raw_cabbage.clone()));

        COLLECTOR.roots.borrow_mut().push(raw_cabbage);

        CabbageBox {
            raw_cabbage: self.raw_cabbage.clone(),
            _type: std::marker::PhantomData,
            is_root: self.is_root,
        }
    }
}

impl<T> Drop for CabbageBox<T> {
    fn drop(&mut self) {
        // roots 객체 목록에서 제거
        if self.is_root {
            let mut roots = COLLECTOR.roots.borrow_mut();

            // 포인터 값이 일치하는 것 하나만 제거
            let mut index = -1;

            for (i, obj) in roots.iter().enumerate() {
                if obj.borrow().data_ptr == self.raw_cabbage.data_ptr {
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
