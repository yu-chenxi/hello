use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

type NodePtr = Option<Rc<RefCell<Node>>>;
#[derive(Debug)]
struct Node {
    data: i32,
    next: NodePtr,
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("{} Dropping!", self.data);
    }
}

// Rc only used in single thread
// RefCell only used in single thread
fn main() {
    let first = Rc::new(RefCell::new(Node {
        data: 1,
        next: None,
    }));

    let second = Rc::new(RefCell::new(Node {
        data: 2,
        next: None,
    }));
    // RefCell<T> fn
    first.borrow_mut().next = Some(second.clone()); // Rc<T> fn
    /*
    first -> [1][]  rc = 1 -> 0
                |
    second -> [2][] rc = 2 -> 1 -> 0
    */
    // second.borrow_mut().next = Some(first.clone());
    /*
    first -> [1][]  rc = 2
                ||
    second -> [2][] rc = 2
    */

    // shit dark dark
    // let mut first = Box::new(Node {
    //     data: 2,
    //     next: None,
    // });

    // let mut second = Box::new(Node {
    //     data: 2,
    //     next: None,
    // });
    // first.next = Some(second);
    // second.next = Some(first);

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let msg = String::from("hi!");
        tx.send(msg).unwrap();
    });

    let msg = rx.recv().unwrap();
    println!("msg = {}", msg);

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    const N: u32 = 100;
    for _ in 0..N {
        let counter = counter.clone();
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let counter = counter.lock().unwrap();
    println!("counter = {}", counter);
    // let ok = false;
    // let name = "mike";
    // assert!(ok, "It's false! {}", name);
}

#[allow(dead_code)]
fn add_two(x: i32) -> i32 {
    x + 2
}

#[cfg(test)]
mod tests {
    use crate::add_two;

    #[test]
    fn it_works() -> Result<(), String> {
        if add_two(2) == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
