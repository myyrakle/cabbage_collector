use std::{cell::UnsafeCell, sync::LazyLock};

pub struct ObjectPool {}

#[derive(Debug, Clone)]
pub struct RawCabbage {
    pub marked: bool,
    pub size: usize,
    pub data_ptr: usize,
    pub child_objects: Vec<*mut RawCabbage>,
}

impl RawCabbage {
    pub fn allocate<T>(value: T) -> Self {
        let boxed_obj = Box::new(value);
        let ptr = Box::into_raw(boxed_obj);

        RawCabbage {
            marked: false,
            size: std::mem::size_of::<T>(),
            data_ptr: ptr as usize,
            child_objects: Vec::new(),
        }
    }

    pub fn get_data_ref<T>(&self) -> &T {
        unsafe { &*(self.data_ptr as *const T) }
    }

    pub fn get_data_mut<T>(&mut self) -> &mut T {
        unsafe { &mut *(self.data_ptr as *mut T) }
    }
}

pub trait CabbageObject {
    fn get_child_objects(&self) -> Vec<*mut dyn CabbageObject>;

    fn mark(&mut self, marked: bool);
    fn is_marked(&self) -> bool;
    fn free(&mut self);
}

pub struct CabbageCollector {
    roots: Vec<RawCabbage>,
}

impl CabbageCollector {
    pub fn new_collector() -> Self {
        CabbageCollector { roots: Vec::new() }
    }

    pub fn allocate_to_roots<T>(&mut self, value: T) -> RawCabbage {
        let raw_cabbage = RawCabbage::allocate(value);

        self.roots.push(raw_cabbage.clone());

        raw_cabbage
    }

    pub fn allocate_under_parent(
        &mut self,
        parent: &mut RawCabbage,
        value: RawCabbage,
    ) -> RawCabbage {
        let raw_cabbage = RawCabbage::allocate(value);

        parent
            .child_objects
            .push(raw_cabbage.data_ptr as *mut RawCabbage);

        raw_cabbage
    }
}
