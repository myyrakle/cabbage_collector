use cabbage_collector::{CabbageBox, CabbageCollector};

fn main() {
    let mut collector = CabbageCollector::new_collector();

    let mut obj = CabbageBox::new(42);

    println!("{:?}", obj);
}
