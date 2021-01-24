use std::{process, time::SystemTime};

fn inc_n(delta: i32) -> impl Fn(i32) -> i32 {
    move |n: i32| n + delta
    // borrow with ref
}

fn foo() {
    let inc = inc_n(3);
    let r = inc(2);
    println!("{}", r);
}

fn foo2() {
    let s = "hello".to_string();
    let c = || s; // capture with owership
    c();
    // println!("{}", s); moved
    let mut n = 1;
    let mut c2 = || n += 1; // capture with &mut n
    c2();
    println!("{}", n);
}

// trait compound
trait Eat {
    fn eat(&self);
}

trait Mov {
    fn mov(&self);
}

struct Dog {
    name: String,
}

impl Eat for Dog {
    fn eat(&self) {
        println!("{} eats bones.", self.name);
    }
}

impl Mov for Dog {
    fn mov(&self) {
        println!("{} can run.", self.name);
    }
}

fn static_disp<T: Eat + Mov>(em: &T) {
    em.eat();
    em.mov();
}

fn foo3() {
    let dog = Dog {
        name: "mike".to_string(),
    };
    static_disp(&dog);
    eprintln!("hello rust!");
    let this_file = file!();
    println!("defined file in: {}", this_file);
    foo3_2();
}

fn foo3_2() {
    // in place
    let mut v = vec![2, 5, 11];
    println!("v = {:?}", v);
    let s: i32 = v.iter().sum();
    println!("s = {}", s);
    v.iter_mut().for_each(|e| *e += 1);
    println!("v = {:?}", v);
    let s = v.iter().fold(1, |acc, e| acc * e);
    println!("s = {}", s);
}

fn foo4() {
    let now = SystemTime::now();
    // println!("do someing..."); 39us
    // const SIZ: usize = 1024; 5us
    // let _v = Vec::<i32>::with_capacity(SIZ);
    // let cnt = AtomicU32::new(0); // Ordering::SeqCst
    // const N: u32 = 1000000;
    // for _ in 0..N {
    //     cnt.fetch_add(1, Ordering::Relaxed);
    // }
    let elapse = now.elapsed().unwrap(); // time counting
    println!("{:?}", elapse);
    // process::exit(3i32);
}

fn main() {
    foo();
    foo2();
    foo3();
    foo4();
}

// boxed closure => unboxed closure
// method receiver => object
// Fn: &self
// FnMut: &mut self
// FnOnce: self
// unsound: a. 不健全的
