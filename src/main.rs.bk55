#![allow(dead_code)]

use std::{
    mem::size_of_val,
    sync::atomic::{AtomicU32, Ordering},
};

use std::sync::Arc;

fn inc_n(counter: Arc<AtomicU32>) {
    const DELTA: u32 = 1000;
    for _ in 0..DELTA {
        // counter.deref()
        counter.fetch_add(1, Ordering::Relaxed);
    }
    // counter.strong -= 1
}

fn foo() {
    const NTHR: u32 = 3;
    let counter = Arc::new(AtomicU32::new(0));
    let mut handles = vec![];

    for _ in 0..NTHR {
        let counter = counter.clone();
        let handle = std::thread::spawn(move || {
            inc_n(counter);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("counter = {}", counter.load(Ordering::Relaxed));
}

fn main() {
    // foo();
    // foo2();
    prime();
}

fn bar<F: Fn()>(f: F)
where
    F: Send + 'static,
{
    f();
}

fn foo2() {
    let s = "john".to_string();
    let s2 = s.clone();

    let c = || {
        println!("hello, {}", s);
    };
    let c2 = move || {
        println!("hello, {}", s2);
    };

    let n = 13;
    let c3 = move || {
        println!("{}", n);
    };

    println!("c size = {}", size_of_val(&c));
    println!("c2 size = {}", size_of_val(&c2)); // size_of::<String>()
                                                // bar(c);
    println!("c3 size = {}", size_of_val(&c3));
    bar(c2);
    bar(c3);

    // thread::spawn(c2);
    // thread::spawn(c); // may dangling pointer
    // prime();
}

fn set_b(w: u64, n: u32) -> u64 {
    w | 1 << n
}

fn clear_n(w: u64, n: u32) -> u64 {
    w & !(1 << n)
}

#[test]
fn test_bit() {
    assert_eq!(1, set_b(0, 0));
    assert_eq!(3, set_b(1, 1));
    assert_eq!(5, clear_n(7, 1));
}

fn prime() {
    const N: usize = 1000_0000;
    let mut flag = vec![std::u64::MAX; N / 64 + 1];
    let m = (N as f64).sqrt() as usize;
    for i in 2..=m {
        let x = i / 64;
        let y = i % 64;
        let b = flag[x] & (1 << y);
        if b != 0 {
            let start = i * i;
            for j in (start..=N).step_by(i) {
                let x = j / 64;
                let y = j % 64;
                flag[x] = clear_n(flag[x], y as u32);
            }
        }
    }

    for i in 2..=N {
        let x = i / 64;
        let y = i % 64;
        let b = flag[x] & (1 << y);
        if b != 0 {
            println!("{}", i);
        }
    }
}

fn prime2() {
    assert_eq!(1, std::mem::size_of::<bool>());
    const N: usize = 1000_0000;
    let mut flag = vec![true; N + 1];
    let m = (N as f64).sqrt() as usize;
    for i in 2..=m {
        if flag[i] {
            let start = i * i;
            for j in (start..=N).step_by(i) {
                flag[j] = false;
            }
        }
    }

    for i in 2..=N {
        if flag[i] {
            println!("{}", i);
        }
    }
    // const SECS: u64 = 120;
    // std::thread::sleep(Duration::from_secs(SECS));
}