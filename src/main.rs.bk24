use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

#[derive(Debug, Clone, Copy)]
struct Pt {
    x: i32,
    y: i32,
}

impl Pt {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

fn main() {
    let pt = Arc::new(Mutex::new(Pt::new(1, 2)));
    const NUM: u32 = 3;
    let mut ts = vec![];
    for _ in 0..NUM {
        let pt = Arc::clone(&pt);
        // multi-threads point to single memory-area
        let t = thread::spawn(move || {
            let mut pt = pt.lock().unwrap();
            pt.x += 1; // ptr
            pt.y += 2;
        });
        ts.push(t);
    }

    for t in ts {
        t.join().unwrap();
    }

    println!("{:?}", pt.lock().unwrap());
}
