use cabbage_collector::{COLLECTOR, CabbageBox};

fn main() {
    {
        let obj = CabbageBox::new(42);

        COLLECTOR.run_cabbage_collection();

        println!("{:?}", obj);
    }
}
