#[derive(Debug, Clone)]
pub struct RawCabbage {
    pub marked: bool,
    pub size: usize,
    pub data_ptr: usize,
    pub child_objects: Vec<*mut RawCabbage>,
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

    pub fn get_data_ref<T>(&self) -> &T {
        unsafe { &*(self.data_ptr as *const T) }
    }

    pub fn get_data_mut<T>(&mut self) -> &mut T {
        unsafe { &mut *(self.data_ptr as *mut T) }
    }
}
