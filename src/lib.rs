use std::sync::{LazyLock, Mutex};

use raw::RawCabbage;

mod cabbage_box;
mod raw;

pub use cabbage_box::CabbageBox;

pub struct CabbageCollector {
    roots: Mutex<Vec<RawCabbage>>,
    all_objects: Mutex<Vec<RawCabbage>>,
}

impl CabbageCollector {
    pub fn new_collector() -> Self {
        CabbageCollector {
            roots: Mutex::new(Vec::new()),
            all_objects: Mutex::new(Vec::new()),
        }
    }

    pub fn allocate_to_roots<T>(&self, value: T) -> RawCabbage {
        let raw_cabbage = RawCabbage::allocate(value);

        self.roots.lock().unwrap().push(raw_cabbage.clone());
        self.all_objects.lock().unwrap().push(raw_cabbage.clone());

        raw_cabbage
    }

    pub fn allocate_under_parent(&self, parent: &mut RawCabbage, value: RawCabbage) -> RawCabbage {
        let raw_cabbage = RawCabbage::allocate(value);

        parent
            .child_objects
            .push(raw_cabbage.data_ptr as *mut RawCabbage);

        self.all_objects.lock().unwrap().push(raw_cabbage.clone());

        raw_cabbage
    }

    pub fn run_cabbage_collection(&self) {
        // STEP 1. 모든 객체에 Mark=false 초기화
        self.reset_mark();
    }

    fn reset_mark(&self) {
        let mut all_objects = self.all_objects.lock().unwrap();
        for obj in all_objects.iter_mut() {
            obj.marked = false;
        }
    }
}

pub static COLLECTOR: LazyLock<CabbageCollector> =
    LazyLock::new(|| CabbageCollector::new_collector());
