use std::{
    cell::RefCell,
    fmt::Debug,
    fs::File,
    io::BufWriter,
    io::Error,
    io::{stdin, Write},
    mem,
    rc::Rc,
    sync::{Arc, Mutex},
    thread,
};

use mem::size_of_val;

#[cfg(test)]
fn prime(n: u32) -> bool {
    let r = n % 6;
    if n >= 6 && r != 1 && r != 5 {
        return false;
    }

    let m = (n as f64).sqrt() as u32;
    for i in 2..=m {
        if n % i == 0 {
            return false;
        }
    }
    true
}

#[test]
fn test_prime() {
    let tests = [
        (true, 2),
        (true, 17),
        (false, 24),
        (false, 169),
        (true, 65537),
    ];
    for test in &tests {
        assert_eq!(test.0, prime(test.1));
    }
}

// use rust-analyzer
fn foo() {
    let x = 3; // 3 -> enter -> . -> let
    let rx = Box::new(x); // x.box
    println!("{}", rx);
}

fn foo2() -> Result<(), Error> {
    let file = File::create("./my.txt")?;
    let mut buf = BufWriter::new(file);
    writeln!(buf, "{}", "hello buffer!")?;
    buf.flush()
}

fn foo3() {
    let v = vec![1, 2, 3];
    let v_it = v.iter();
    let s = v_it.sum::<i32>();
    println!("{}", s);
    let v_it = v.iter();
    let s = v_it.fold(0, |acc, e| acc + e * e);
    println!("{}", s);
}

fn my_div(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}

fn foo4() {
    let x = 3.0;
    let y = 0.0;
    match my_div(x, y) {
        Some(r) => println!("{}", r),
        None => println!("div 0!"),
    }
}

macro_rules! my_max {
    ($a:expr, $b:expr) => {
        if $a > $b {
            $a
        } else {
            $b
        }
    };
}

#[allow(dead_code)]
fn foo5() {
    let mut ln = String::new();
    let n = stdin().read_line(&mut ln).unwrap();
    let a = ln[..n - 1].parse::<i32>().unwrap();
    ln.clear();
    let n = stdin().read_line(&mut ln).unwrap();
    let b = ln[..n - 1].parse::<i32>().unwrap();
    println!("max of {}, {} is {}", a, b, my_max!(a, b));
    println!("hello a = {}", a); // "string literal".println
}

// ctrl-shift-o: fuzzy search symbol in a file
fn main() {
    foo();
    foo2().unwrap();
    foo3();
    foo4();
    // foo5();
    foo6(); // fuzz select
    foo7();
    foo8();
    foo9();
    foo10();
    foo11();
}

fn foo6() {
    let mut stack = vec![];
    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

fn foo7() {
    let age: Result<u8, _> = "34".parse();
    if let Ok(age) = age {
        println!("{}", age);
    }
}

fn foo8() {
    let v = vec!['a', 'b', 'c'];
    for (idx, val) in v.iter().enumerate() {
        println!("{} is at index {}", val, idx);
    }
    let (_x, _y, _z) = (1, 2, 3);
    let pt = (3, 5);
    fn coord(&(x, y): &(i32, i32)) {
        // ref of tuple
        println!("{} {}", x, y);
    }
    coord(&pt);
}

fn foo9() {
    let x = 'c';
    let y = 'X';
    fn judge(c: char) {
        let s = match c {
            'a'..='z' => "upper case",
            'A'..='Z' => "lower case",
            _ => "other letters",
        };
        println!("{}", s);
    }
    judge(x);
    judge(y);
    struct Pt {
        x: i32,
        y: i32,
    };
    let p1 = Pt { x: 3, y: 7 };
    let Pt { y, x } = p1;
    println!("{} {}", x, y);
    let Pt { y: a, x: b } = p1;
    println!("{} {}", a, b); // 7 3
                             // let p2 = Pt { x: 5, y: 0 };
    let p2 = Pt { x: 1, y: 3 };
    match p2 {
        Pt { x, y: 0 } => println!("On the x axis at {}", x),
        Pt { x: 0, y } => println!("On the y axis at {}", y),
        Pt { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
    foo9_2();
    match p2 {
        Pt { x, .. } => println!("{}", x),
    };
}

fn foo9_2() {
    fn inc(n: i32, _: i32) -> i32 {
        n + 1
    }
    type Bop = fn(i32, i32) -> i32;
    let bop: Bop = inc;
    println!("{}", bop(3, 1));
}

fn foo10() {
    let x = 3;
    let sx = Rc::new(RefCell::new(x)); // runtime borrow check
    {
        let mut rx = sx.borrow_mut();
        *rx += 4;
        // {value(ptr), borrow(ptr)}
        println!("{}", size_of_val(&rx)); // rx.borrow
                                          // rx dropped, update sx.borrow
                                          // sx.borrow == rx.borrow
    }
    // let mut rx2 = sx.borrow_mut(); // 'already borrowed: BorrowMutError'
    // *rx2 += 5;
    let rx2 = sx.borrow();
    println!("{}", *rx2);
    let v = vec![2, 5, 11];
    let it = v.iter(); // Iter{ptr, end}
    println!("{}", mem::size_of_val(&it));
    println!("{}", mem::size_of::<RefCell<i32>>()); // RefCell{borrow, ptr}
}

#[derive(Debug, Clone, Copy)]
struct S<T, T2>
// generic decl in struct right, in fn left
where
    T: Debug,
    T2: Debug,
{
    x: T,
    y: T2,
}

fn foo11() {
    let s1 = S { x: -1, y: 3.14 };
    let s2: S<i32, _> = s1; // _: auto deducation
    println!("{:?}", s2);
    foo11_2();
}

fn foo11_2() {
    const N: u32 = 5;
    let mut hs = Vec::with_capacity(N as usize);
    let cnt = Arc::new(Mutex::new(0));

    for i in 0..N {
        let cnt = cnt.clone();
        let h = thread::spawn(move || {
            let mut n = cnt.lock().unwrap();
            *n += 1; // store in stack
            println!("thread {}, n = {}", i, n);
        });
        hs.push(h);
    }

    hs.into_iter().for_each(|h| h.join().unwrap()); // for_each: consuming adapter
                                                    // iter: &
                                                    // iter_mut: &mut
                                                    // into_iter: owership
    println!("cnt = {}", cnt.lock().unwrap());
}
