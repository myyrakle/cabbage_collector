pub struct ObjectPool {}

pub trait CabbageObject {
    fn get_child_objects(&self) -> Vec<*mut dyn CabbageObject>;
}

pub struct CabbageCollector {
    roots: Vec<*mut dyn CabbageObject>,
}
