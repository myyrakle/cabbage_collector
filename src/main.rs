#![allow(unused)]

use cabbage_collector::{COLLECTOR, CabbageBox};

#[derive(Debug, Clone)]
struct SampleChild {
    pub value: CabbageBox<i32>,
}

fn step1() {
    println!("----------------");
    println!("---- STEP 1 ----");

    {
        #[derive(Debug, Clone)]
        struct A {
            pub value: i32,
        }

        let child_obj = CabbageBox::new(A { value: 1 });
        println!("{:?}", child_obj);
    }
    println!("Before GC");
    COLLECTOR.print_for_debug();
    COLLECTOR.run_cabbage_collection();
    println!("After GC");
    COLLECTOR.print_for_debug();
}

fn step2() {
    println!("----------------");
    println!("---- STEP 2 ----");

    {
        #[derive(Debug, Clone)]
        struct A {
            pub value: i32,
        }

        let child_obj = CabbageBox::new(A { value: 1 });
        println!("{:?}", child_obj);

        let cloned_obj = child_obj.clone();
        println!("{:?}", cloned_obj);
    }
    println!("Before GC");
    COLLECTOR.print_for_debug();
    COLLECTOR.run_cabbage_collection();
    println!("After GC");
    COLLECTOR.print_for_debug();
}

fn main() {
    // step1();

    step2();

    // {
    //     let mut parent_obj = CabbageBox::new(SampleChild {
    //         value: CabbageBox::non_root(&CabbageBox::new(1)),
    //     });

    //     println!("parent_obj: {:?}", parent_obj);

    //     COLLECTOR.run_cabbage_collection();
    // }
}
