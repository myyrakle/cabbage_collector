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

        COLLECTOR.print_for_debug();
    }
    println!("Before GC");
    COLLECTOR.print_for_debug();
    COLLECTOR.run_cabbage_collection();
    println!("After GC");
    COLLECTOR.print_for_debug();
}

fn step3() {
    println!("----------------");
    println!("---- STEP 3 ----");

    {
        #[derive(Debug, Clone)]
        struct Child {
            pub value: i32,
        }

        #[derive(Debug, Clone)]
        struct Parent {
            pub child: CabbageBox<Child>,
        }

        let child_obj = CabbageBox::new(Child { value: 1 });

        let parent_obj = CabbageBox::new(Parent {
            child: CabbageBox::non_root(&child_obj),
        });

        COLLECTOR.print_for_debug();
    }
    println!("Before GC");
    COLLECTOR.print_for_debug();
    COLLECTOR.run_cabbage_collection();
    println!("After GC");
    COLLECTOR.print_for_debug();
}

// circular reference
fn step4() {
    println!("----------------");
    println!("---- STEP 4 ----");

    {
        #[derive(Debug, Clone)]
        struct A {
            pub value: Option<CabbageBox<B>>,
        }

        #[derive(Debug, Clone)]
        struct B {
            pub value: Option<CabbageBox<A>>,
        }

        let mut a_obj = CabbageBox::new(A { value: None });
        let mut b_obj = CabbageBox::new(B { value: None });

        a_obj.value = Some(b_obj.clone());
        b_obj.value = Some(a_obj.clone());

        COLLECTOR.print_for_debug();
    }
    println!("Before GC");
    COLLECTOR.print_for_debug();
    COLLECTOR.run_cabbage_collection();
    println!("After GC");
    COLLECTOR.print_for_debug();
}

fn main() {
    // step1();

    // step2();

    // step3();

    step4();
}
