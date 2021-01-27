#![allow(dead_code)]
use std::u64;

type PNode = Option<Box<Node>>;
struct Node {
    val: i32,
    next: PNode,
}

impl Node {
    fn travse(&self) {
        println!("{}", self.val);
        if let Some(ref pn) = self.next {
            pn.travse();
        }
    }

    fn last_mut(&mut self) -> &mut Node {
        if let Some(ref mut pn) = self.next {
            pn.last_mut()
        } else {
            self
        }
    }
}

struct List {
    head: PNode,
}

impl List {
    fn new() -> List {
        List { head: None }
    }

    fn push(&mut self, val: i32) {
        let ph = self.head.take();
        let pn = Box::new(Node { val, next: ph });
        self.head = Some(pn);
    }

    fn pop(&mut self) -> Option<i32> {
        let ph = self.head.take();
        match ph {
            Some(pn) => {
                self.head = pn.next; // head -> second node
                Some(pn.val)
            }
            None => None,
        }
    }

    fn travse(&self) {
        match self.head {
            Some(ref pn) => pn.travse(),
            None => (),
        }
    }
}

fn foo2() {
    let mut st = List::new();
    st.push(1);
    st.push(2);
    st.push(3);
    st.travse();
    while let Some(val) = st.pop() {
        println!("{}", val);
    }
}

fn foo3() {
    let x = Some(5);
    assert_eq!(x.is_some(), true);
    assert_eq!(x.is_none(), false);
    let x: Option<i32> = None;
    assert_eq!(x.is_none(), true);
    let mut x = Some(7);
    let y = x.take();
    assert_eq!(x, None);
    assert_eq!(y, Some(7));
    let mut x = Some(9);
    let y = x.replace(11);
    assert_eq!(x, Some(11));
    assert_eq!(y, Some(9));
    // --------------------
    let h = "hello".to_owned();
    let mut s = Some(h);
    let s2 = s.take();
    assert_eq!(s2, Some("hello".to_string()));

    let mut p1 = Box::new(Node { val: 1, next: None });
    for i in 2..=4 {
        let p = Box::new(Node {
            val: i,
            next: p1.next.take(),
        });
        p1.next = Some(p);
    }
    let lm = p1.last_mut();
    let p = Box::new(Node {
        val: 13,
        next: None,
    });
    lm.next = Some(p);
    p1.travse();
}

fn main() {
    // foo();
    // foo2();
    foo3();
}

fn foo() {
    prime(30);
    prime(100_0000);
}

fn prime(n: u32) {
    let t = n + 1;
    let len = (t + 1) / 64 + (t % 64 != 0) as u32;
    let mut flags = vec![u64::MAX; len as usize];
    let m = (n as f64).sqrt() as u32;
    for i in 2..=m {
        let (x, y) = ((i / 64) as usize, i % 64);
        let b = flags[x] & (1 << y);
        if b != 0 {
            let start = i * i;
            for j in (start..=n).step_by(i as usize) {
                let (x, y) = ((j / 64) as usize, j % 64);
                flags[x] &= !(1 << y);
            }
        }
    }

    for i in 2..=n {
        let (x, y) = ((i / 64) as usize, i % 64);
        let b = flags[x] & (1 << y);
        if b != 0 {
            println!("{}", i);
        }
    }
}
