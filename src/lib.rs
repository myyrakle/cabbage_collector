use std::sync::LazyLock;

pub struct ObjectPool {}

pub trait CabbageObject {
    fn get_child_objects(&self) -> Vec<*mut dyn CabbageObject>;

    fn mark(&mut self, marked: bool);
    fn is_marked(&self) -> bool;
    fn free(&mut self);
}

pub struct CabbageCollector {
    roots: Vec<*mut dyn CabbageObject>,
}

pub static mut GC: LazyLock<CabbageCollector> =
    LazyLock::new(|| CabbageCollector { roots: Vec::new() });
