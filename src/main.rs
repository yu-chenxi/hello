use std::{cell::RefCell, mem, rc::Rc};

fn hello() -> Box<String> {
    Box::new(String::from("hello rust!"))
}

fn foo3() {
    let s = hello();
    println!("{}", s);
}

fn foo2() {
    let v = Vec::from([1, 2, 3]);
    println!("v = {:?}", v);
    let x = Rc::new(RefCell::new(1));
    let y = x.clone();
    let z = Rc::downgrade(&x);
    let mut rx = x.borrow_mut(); // RefMut<i32>
    *rx += 1;
    println!("x = {:?}", x);
    println!(
        "strong_count = {}, weak_count = {}",
        Rc::strong_count(&y),
        z.weak_count()
    );
    // z -> Weak
    //      |
    // x -> Rc
    // y -> Rc
}

fn main() {
    foo();
    foo2();
    foo3();
}

fn foo() {
    let mut v = Vec::new();
    let mut cap = v.capacity();
    const N: u32 = 100;
    println!("init cap = {}", cap);
    for e in 0..N {
        v.push(e);
        let _cap = v.capacity();
        if _cap != cap {
            const E_SIZE: usize = mem::size_of::<i32>();
            const KB: f64 = 1024.0;
            let free = cap * E_SIZE;
            let free = (free as f64) / KB;
            let alloc = _cap * E_SIZE;
            let alloc = (alloc as f64) / KB;
            println!(
                "expanded cap = {} (free {:.2} K, alloc {:.2} K)",
                _cap, free, alloc
            );
            cap = _cap;
        }
    }
    v.sort_by(|a, b| b.cmp(a));
}
