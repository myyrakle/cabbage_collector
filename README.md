# Cabbage Collector

- Simple GC implementation
- Simple Mark and Sweep

## Guide

The way to create and use the object is almost the same as `Box<T>`.
However, in the current implementation, GC must be triggered manually.

```rust
    {
        #[derive(Debug, Clone)]
        struct A {
            pub value: i32,
        }

        let child_obj = CabbageBox::new(A { value: 1 });
        println!("{:?}", child_obj);
    }
    COLLECTOR.run_cabbage_collection();
```

The circular reference issue has been resolved.

```rust
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
    }
    COLLECTOR.run_cabbage_collection();
```

## Checklist

- [x] Circular Reference
- [ ] Automatically identifies root and non-root
- [ ] Auth trigger GC
- [ ] Concurrent GC
- [ ] Generational GC
- [ ] Memory Compaction
