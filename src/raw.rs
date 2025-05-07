#[derive(Debug, Clone)]
pub struct RawCabbage {
    pub(crate) data_ptr: usize,
    pub(crate) layout: std::alloc::Layout,
    pub(crate) marked: bool,
    pub(crate) child_objects: Vec<*mut RawCabbage>,
    pub(crate) is_root: bool,
}

unsafe impl Send for RawCabbage {}

impl RawCabbage {
    pub fn allocate<T>(value: T) -> *mut Self {
        let boxed_obj = Box::new(value);
        let ptr = Box::into_raw(boxed_obj) as usize;
        let layout = std::alloc::Layout::new::<T>();

        let raw_cabbage = RawCabbage {
            marked: false,
            layout,
            data_ptr: ptr,
            child_objects: Vec::new(),
            is_root: false,
        };

        Box::into_raw(Box::new(raw_cabbage)) as *mut Self
    }

    pub fn deallocate(&mut self) {
        unsafe {
            std::alloc::dealloc(self.data_ptr as *mut u8, self.layout);

            // TODO: call Drop function (Drop trait)
        }
    }

    pub fn get_data_ref<T>(&self) -> &T {
        unsafe { &*(self.data_ptr as *const T) }
    }

    pub fn get_data_mut<T>(&mut self) -> &mut T {
        unsafe { &mut *(self.data_ptr as *mut T) }
    }
}
