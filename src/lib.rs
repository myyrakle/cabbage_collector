use std::{cell::RefCell, sync::LazyLock};

pub use raw::RawCabbage;

mod cabbage_box;
mod raw;

pub use cabbage_box::CabbageBox;

pub struct CabbageCollector {
    pub(crate) roots: RefCell<Vec<*mut RawCabbage>>,
    pub(crate) all_objects: RefCell<Vec<*mut RawCabbage>>,
}

unsafe impl Send for CabbageCollector {}
unsafe impl Sync for CabbageCollector {}

impl CabbageCollector {
    pub fn new_collector() -> Self {
        CabbageCollector {
            roots: RefCell::new(Vec::new()),
            all_objects: RefCell::new(Vec::new()),
        }
    }

    pub fn run_cabbage_collection(&self) {
        // STEP 1. 모든 객체에 Mark=false 초기화
        self.reset_mark();

        // STEP 2. Mark Phase
        self.mark();

        // STEP 3. Sweep Phase
        self.sweep();
    }

    fn reset_mark(&self) {
        for obj in self.all_objects.borrow_mut().iter() {
            unsafe {
                (**obj).marked = false;
            }
        }
    }

    fn mark(&self) {
        for root in self.roots.borrow_mut().iter().cloned() {
            unsafe {
                (*root).marked = true;

                Self::mark_recursion(root);
            }
        }
    }

    fn mark_recursion(obj: *mut RawCabbage) {
        unsafe {
            if (*obj).marked {
                return;
            }
        }

        unsafe {
            (*obj).marked = true;
        }

        unsafe {
            for child in (*obj).child_objects.iter().cloned() {
                Self::mark_recursion(child);
            }
        }
    }

    fn sweep(&self) {
        self.all_objects.borrow_mut().retain(|obj| unsafe {
            if (**obj).marked {
                true
            } else {
                COLLECTOR.roots.borrow_mut().retain(|root| {
                    let root_ptr = (**root).data_ptr;
                    let obj_ptr = (**obj).data_ptr;

                    root_ptr != obj_ptr
                });

                RawCabbage::deallocate(*obj);
                false
            }
        });
    }

    #[allow(dead_code)]
    pub fn print_for_debug(&self) {
        println!("roots: {:?}", self.roots.borrow());
        println!("all_objects: {:?}", self.all_objects.borrow());
    }
}

pub static COLLECTOR: LazyLock<CabbageCollector> = LazyLock::new(CabbageCollector::new_collector);
