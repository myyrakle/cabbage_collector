use cabbage_collector::CabbageCollector;

fn main() {
    let mut collector = CabbageCollector::new_collector();

    let mut obj = collector.allocate_to_roots(4343);

    let data = obj.get_data_ref::<i32>();
    println!("{}", data);

    let mut_ref = obj.get_data_mut::<i32>();
    *mut_ref = 1234;

    let data = obj.get_data_ref::<i32>();
    println!("{}", data);
}
