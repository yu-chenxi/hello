use std::{
    cell::RefCell,
    mem,
    ptr::swap,
    rc::{Rc, Weak},
};

#[cfg(test)]
mod my {
    use super::*;

    macro_rules! make_test {
        ($n:expr) => {
            (($n as f64).sqrt(), $n)
        };
    }
    #[test]
    fn test_my_sqrt() {
        let tests = vec![
            make_test!(2.0),
            make_test!(13.0),
            make_test!(65537.0),
            make_test!((2 << 28 - 5) as f64),
        ];
        for (act, x) in tests {
            let rt = my_sqrt(x);
            let diff = (act - rt).abs();
            assert!(diff < 1e-6);
        }
        // Err("unknown error!")
    }
}

fn my_sqrt(x: f64) -> f64 {
    const N: u32 = 16;
    let mut rt = x / 2.0;
    for _ in 0..N {
        rt = (rt + x / rt) / 2.0;
    }
    rt
}

fn foo() {
    println!("{:.3}", my_sqrt(2.0));
    println!("{:.3}", my_sqrt(17.0));
    println!("{:.3}", my_sqrt(65537.0));
}

fn main() {
    foo();
    foo2();
    foo3();
    foo4();
    foo5();
}
// alt-up: move line up
// alt-down: move line down
// shift-alt-up: copy line up
// ctrl-shift-\: {} ()

enum E<'a> {
    _Move(i32, i32),
    _Msg(&'a str),
}

mod my2 {
    #[derive(Debug)]
    pub struct Person<'a> {
        name: &'a str,
        age: u32,
    }

    impl<'a> Person<'a> {
        pub fn new(name: &'a str, age: u32) -> Self {
            Self { name, age }
        }
        pub fn get_name(&self) -> &str {
            self.name
        }
        pub fn set_age(&mut self, age: u32) {
            self.age = age;
        }
    }
}

use my2::Person;

fn foo2() {
    println!("{}", mem::size_of::<E>()); // tag + ptr + len = 3 * usize = 24
    let rname: &str;
    // {
    let name = "mike".to_string();
    rname = &name;
    // }
    let mut per = Person::new(&rname, 18);
    println!("name = {}", per.get_name());
    per.set_age(20);
    println!("{:?}", per);
}

type NodePtr = Option<Rc<RefCell<Node>>>;
type WNodePtr = Option<Weak<RefCell<Node>>>;

struct Node {
    data: i32,
    prev: WNodePtr,
    next: NodePtr,
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("{} dropped", self.data);
    }
}

fn foo3() {
    // break cyclic ref
    let p1 = Rc::new(RefCell::new(Node {
        data: 1,
        prev: None,
        next: None,
    }));
    let p2 = Rc::new(RefCell::new(Node {
        data: 2,
        prev: None,
        next: None,
    }));
    let p3 = Rc::new(RefCell::new(Node {
        data: 3,
        prev: None,
        next: None,
    }));

    p1.borrow_mut().next = Some(p2.clone());
    p2.borrow_mut().prev = Some(Rc::downgrade(&p1));
    p2.borrow_mut().next = Some(p3.clone());
    p3.borrow_mut().prev = Some(Rc::downgrade(&p1));
    // Rc:
    // strong
    // weak
    // value: T
    assert_eq!(1, Rc::strong_count(&p1));
    assert_eq!(2, Rc::strong_count(&p2));
    assert_eq!(1, Rc::strong_count(&p1));
}

fn gcd(mut a: u32, mut b: u32) -> u32 {
    if a < b {
        unsafe {
            swap(&mut a, &mut b);
        }
    }

    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

#[allow(clippy::many_single_char_names)]
fn gcd2(a: u32, b: u32) -> u32 {
    let (mut x, mut y) = if a > b { (a, b) } else { (b, a) };
    while y != 0 {
        let r = x % y;
        x = y;
        y = r;
    }
    x
}

fn foo4() {
    assert_eq!(3, gcd(12, 15));
    assert_eq!(7, gcd(21, 7));
    assert_eq!(6, gcd(24, 18));
    assert_eq!(1, gcd(7, 37));
    // -----------------------
    assert_eq!(3, gcd2(12, 15));
    assert_eq!(7, gcd2(21, 7));
    assert_eq!(6, gcd2(24, 18));
    assert_eq!(1, gcd2(7, 37));
}

fn foo5() {
    let mut x = 3;
    let y = 5;
    let prev_x = mem::replace(&mut x, y);
    println!("x = {}, previous x value = {}", x, prev_x);
}
