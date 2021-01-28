#![allow(dead_code)]

use std::borrow::Cow;

type Link = Option<Box<Node>>;
struct Node {
    elem: i32,
    next: Link,
}

struct List {
    head: Link,
}

impl List {
    fn new() -> List {
        Self { head: None }
    }

    fn push(&mut self, elem: i32) {
        self.head = Some(Box::new(Node {
            elem,
            next: self.head.take(),
        }));
    }

    fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|pnode| {
            self.head = pnode.next;
            pnode.elem
        })
    }

    fn peek_mut(&mut self) -> Option<&mut i32> {
        self.head.as_mut().map(|pnode| &mut pnode.elem)
    }
}

#[cfg(test)]

mod test {
    use super::List;
    use std::mem;

    #[test]
    fn foo() {
        let mut list = List::new();
        assert_eq!(list.pop(), None);
        list.push(1);
        list.push(2);
        list.push(3);
        *list.peek_mut().unwrap() = 4;
        assert_eq!(list.pop(), Some(4));
        list.pop();
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn foo2() {
        let mut x = Some(3);
        // Option::{as_mut, map}
        assert_eq!(x.map(|x| x + 2), Some(5));
        x.as_mut().map(|x| *x += 2);
        assert_eq!(x, Some(5));
        // Option::take
        assert_eq!(x.take(), Some(5));
        assert_eq!(x, None);
        // mem::*
        let mut x = 3;
        let mut y = 2;
        mem::swap(&mut x, &mut y);
        assert_eq!(x, 2);
        assert_eq!(y, 3);
        mem::take(&mut x);
        assert_eq!(x, 0);
        assert_eq!(mem::replace(&mut y, 2), 3);
        assert_eq!(y, 2);
    }
}

fn foo3() {
    let mut cv: Cow<Vec<i32>> = Cow::Owned(vec![1, 2, 3]);
    let cv2 = cv.to_mut(); // &mut T
    cv2[1] = 5;
    println!("{:?}", cv2);
    println!("{:?}", cv);
}

fn main() {
    foo3();
}
