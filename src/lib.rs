use std::{cell::RefCell, rc::Rc, sync::LazyLock};

use raw::RawCabbage;

mod cabbage_box;
mod raw;

pub use cabbage_box::CabbageBox;

pub struct CabbageCollector {
    pub roots: RefCell<Vec<Rc<RefCell<RawCabbage>>>>,
    pub all_objects: RefCell<Vec<Rc<RefCell<RawCabbage>>>>,
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
            obj.borrow_mut().marked = false;
        }
    }

    fn mark(&self) {
        for root in self.roots.borrow_mut().iter() {
            let mut borrowed = root.borrow_mut();
            borrowed.marked = true;

            Self::mark_recursion(borrowed.get_data_mut());
        }
    }

    fn mark_recursion(obj: &mut RawCabbage) {
        if obj.marked {
            return;
        }

        obj.marked = true;

        for child in obj.child_objects.iter_mut() {
            let child = match child.upgrade() {
                Some(child) => child,
                None => {
                    continue;
                }
            };

            let mut borrowed = child.borrow_mut();
            Self::mark_recursion(borrowed.get_data_mut());
        }
    }

    fn sweep(&self) {
        self.all_objects.borrow_mut().retain(|obj| {
            if obj.borrow().marked {
                true
            } else {
                obj.borrow_mut().deallocate();
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
