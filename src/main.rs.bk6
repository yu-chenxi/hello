use std::cell::RefCell;
use std::rc::Rc;

type NodePtr<T> = Option<Rc<RefCell<Node<T>>>>;
struct Node<T> {
    _data: T,
    next: NodePtr<T>,
}

impl<T> Drop for Node<T> {
    fn drop(&mut self) {
        println!("drop!");
    }
}

// clone
// clone + copy
#[derive(Debug, Clone)]
struct A {
    a: i32,
    b: Box<i32>, // Box<T> => = <=> move
}

fn main() {
    // drop first
    let first = Rc::new(RefCell::new(Node {
        _data: 1,
        next: None,
    }));
    // drop second
    let second = Rc::new(RefCell::new(Node {
        _data: 2,
        next: None,
    }));
    first.borrow_mut().next = Some(second.clone());
    second.borrow_mut().next = Some(first.clone());
    let a1 = A {
        a: 1,
        b: Box::new(2),
    };
    let a2 = a1.clone();
    println!("{:?}", a2);
    println!("{:?}", a1);
    let pi = Box::new(1);
    let pi2 = pi.clone();
    println!("{:p}", pi.as_ref() as *const i32);
    println!("{:p}", pi2.as_ref() as *const i32);
    let _pi3 = pi; // the ownership of Box::new(1) is transferred from pi to _pi3
                   // owner rights:
                   // 1. control release
                   // 2. &x(shared), &mut x(exclusive, like C::restrict)
                   // 3. transfer owership
    #[derive(Debug, Copy, Clone)]
    struct B {
        x: i32,
        y: u32,
    }

    let b = B { x: 1, y: 2 };
    let _b2 = b;
    println!("{:?}", b);

    // let x3 = 1;
    // let s2 = "hello".to_string();
    // {
    //     x3;
    //     s2; // move + drop
    // }

    // println!("x3 is {}", x3);
    // println!("s2 is {}", s2);
    let mut v = vec![2, 5, 7, 11, 17];
    rev(&mut v);
    println!("{:?}", v);
}

fn rev(v: &mut [u32]) {
    let mut i = 0;
    let mut j = v.len() - 1;
    while i < j {
        v.swap(i, j);
        i += 1;
        j -= 1;
    }
}

// memory safe != no memory leak
// book-keeping n. 簿记
// owner n. 所有者
/*
value semantic: let y: i32 = x;
reference semantic: let s2 = s1.clone();
shallow copy: stack copy
deep copy: stack + heap copy
Copy: clone() stack
Clone: clone() stack + heap
Move: move
reference: borrowing(借用, 使用权而不是ownership)
self: &self <=> &self
*/
