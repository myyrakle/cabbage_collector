#[derive(Debug, Clone)]
pub struct RawCabbage {
    pub(crate) marked: bool,
    pub(crate) size: usize,
    pub(crate) data_ptr: usize,
    pub(crate) child_objects: Vec<*mut RawCabbage>,
}

unsafe impl Send for RawCabbage {}

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

    pub fn deallocate(&mut self) {
        unsafe {
            let _ = Box::from_raw(self.data_ptr as *mut u8);
        }
    }

    pub fn get_data_ref<T>(&self) -> &T {
        unsafe { &*(self.data_ptr as *const T) }
    }

    pub fn get_data_mut<T>(&mut self) -> &mut T {
        unsafe { &mut *(self.data_ptr as *mut T) }
    }
}
