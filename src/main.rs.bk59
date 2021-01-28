#![allow(dead_code)]

use std::time::SystemTime;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

struct List {
    head: Link,
}

type Link = Option<Box<Node>>;
struct Node {
    elem: i32,
    next: Link,
}

impl List {
    fn new() -> List {
        List { head: None }
    }

    fn push(&mut self, elem: i32) {
        let pn = Box::new(Node {
            elem,
            next: self.h_take(),
        });
        self.head = Some(pn);
    }

    fn pop(&mut self) -> Option<i32> {
        self.h_take().map(|pn| {
            self.head = pn.next;
            pn.elem
        })
    }

    fn peek(&self) -> Option<&i32> {
        // Some(&Box<Node>)
        self.head.as_ref().map(|pn| &pn.elem)
    }

    fn peek_mut(&mut self) -> Option<&mut i32> {
        self.head.as_mut().map(|pn| &mut pn.elem)
    }

    fn h_take(&mut self) -> Link {
        self.head.take()
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();
        assert_eq!(list.pop(), None);
        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        list.push(4);
        list.push(5);
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));
        list.peek_mut().map(|value| *value = 42);
        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Pt {
    x: i32,
    y: i32,
}

impl Pt {
    fn set_dummy(&mut self) {
        self.x = 3;
        self.y = 5;
    }
}

fn set_pt(pt: &mut Pt) {
    pt.x = 3;
    pt.y = 5;
}

fn foo() {
    let mut pt = Some(Pt { x: 1, y: 2 });
    let ppt = pt.as_mut();
    // change the T part of Option<T>
    ppt.map(set_pt);
    assert_eq!(pt, Some(Pt { x: 3, y: 5 }));
    let mut x = Some(3);
    assert_eq!(x.as_ref(), Some(&3));
    x.as_mut().map(|x| *x += 2);
    assert_eq!(x, Some(5));
    // Maps an `Option<T>` to `Option<U>`
    let y = x.map(|x| x as f64);
    assert_eq!(y, Some(5.0));
    // Pt{ x: 3, y: 7}
    // .box(b)
    // .letm(lm)
    let mut ppt = Box::new(Pt { x: 3, y: 7 });
    ppt.set_dummy(); // （&mut ppt).set_dummpy()
                     // Pt::set_dummy(&mut ppt)
    let rx = &mut ppt.x; // like normal ptr
    *rx = 7;
    assert_eq!(*ppt, Pt { x: 7, y: 5 });
}

// super-struct drop first

#[derive(Debug)]
struct S(i32);

impl Drop for S {
    fn drop(&mut self) {
        println!("{:?}", self);
    }
}

#[derive(Debug)]
struct S2 {
    s1: S,
    s3: S,
}

impl Drop for S2 {
    fn drop(&mut self) {
        println!("{:?}", self);
    }
}

fn foo2() {
    let s1 = S(1);
    let s2 = S(2);
    let _s21 = S2 { s1: s1, s3: s2 }; // in order

    {
        let _s13 = S(3);
    }
}

fn foo3() {
    const N: usize = 1_0000;
    let mut v = Vec::with_capacity(N);
    let mut v2 = Vec::with_capacity(N);
    let mut v3 = Vec::with_capacity(N);

    let start = SystemTime::now();
    {
        for _ in 0..N {
            v.push(Box::new([3u64, 8]));
        }

        for _ in 0..N {
            v2.push(Box::new([3u64, 16]));
        }

        for _ in 0..N {
            v3.push(Box::new([3u64, 32]));
        }

        drop(v);
        drop(v2);
        drop(v3);
    }
    let elapse = start.elapsed().unwrap();
    let period = elapse.div_f64((N * 3) as f64);
    println!("{:?} / op(alloc+free)", period);
}

fn main() {
    // foo();
    // foo2();
    foo3();
}

fn foo4() {
    let x = Some(3);
}