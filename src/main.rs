use bumpalo::Bump;
use std::{cell::RefCell, mem, ops::AddAssign, rc::Rc};

fn my_sum<T>(v: &[T]) -> T
where
    T: AddAssign,
    T: Default,
    T: Copy,
{
    let mut s = T::default();
    for e in v {
        s += *e;
    }
    s
}

fn foo4() {
    let v = [2, 3, 5];
    let v2 = [-2.5, 1.3, 0.2];
    let s = my_sum(&v);
    let s2 = my_sum(&v2);
    println!("s = {}", s);
    println!("s2 = {}", s2);
}

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
    foo4();
    foo5(&[1, 2, 4, 8], 0);
    foo6();
    foo7();
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

fn foo5(v: &[i32], idx: usize) {
    if idx < v.len() - 1 {
        foo5(v, idx + 1);
    }
    println!("{}", v[idx]);
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: String, age: u32) -> Self {
        Self { name, age }
    }
}

fn foo6() {
    let arena = Bump::new();
    let p = arena.alloc(Person::new(String::from("mike"), 18));
    println!("{:?}", p);
}

fn foo7() {
    println!("{}", mem::size_of::<Box<[i32]>>()); // ptr + len(special)
    println!("{}", mem::size_of::<Box<(u64, u64)>>()); // ptr
    println!("{}", mem::size_of::<Box<String>>()); // ptr
}
