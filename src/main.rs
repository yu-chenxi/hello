fn swap((x, y): (&str, i32)) -> (i32, &str) {
    (y, x)
}

fn foo2() {
    let t = ("alex", 18);
    let t = swap(t);
    println!("{:?}", t);
}

fn addsub(x: isize, y: isize) -> (isize, isize) {
    (x + y, x - y)
}

fn foo3() {
    let (a, b) = addsub(5, 8);
    println!("a: {}, b: {}", a, b);
}

fn gcd(mut a: u32, mut b: u32) -> u32 {
    if a < b {
        let t = a;
        a = b;
        b = t;
    }

    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

fn foo4() {
    let g = gcd(20, 40);
    assert_eq!(20, g);
}

use std::ops::Mul;
fn s<T: Mul<Output = T>>(x: T, y: T) -> T {
    x * y
}

fn foo5() {
    let a = s(37, 41);
    let b = s::<f32>(37.2, 41.1);
    println!("{}", a);
    println!("{:.6}", b);
}

struct User {
    name: &'static str,
    avatar_url: &'static str,
}

impl User {
    fn show(&self) {
        println!("name: {}", self.name);
        println!("avatar: {}", self.avatar_url);
    }
}

fn foo6() {
    let user = User {
        name: "alex",
        avatar_url: "https://avatar.com/alex",
    };
    user.show();
}

fn math(op: fn(i32, i32) -> i32, a: i32, b: i32) -> i32 {
    op(a, b)
}

fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn product(a: i32, b: i32) -> i32 {
    a * b
}

fn foo7() {
    let (a, b) = (2, 3);
    println!("{}", math(sum, a, b));
    println!("{}", math(product, a, b));
}

fn hello() {
    println!("hello function pointer!");
}

fn foo8() {
    let fn_ptr: fn() = hello; // type: fn()
                              // print func address
    println!("{:p}", fn_ptr);
    let other_fn = hello; // type: fn hello()
                          // println!("{:p}", other_fn);
    hello();
    other_fn();
    fn_ptr();
    (fn_ptr)();
}

type MathOp = fn(i32, i32) -> i32;

fn math2(op: &str) -> MathOp {
    fn sum(a: i32, b: i32) -> i32 {
        a + b
    }
    fn product(a: i32, b: i32) -> i32 {
        a * b
    }
    match op {
        "sum" => sum,
        "product" => product,
        _ => {
            println!("Warning: Not Implemented {} operator, Replace with sum", op);
            sum
        }
    }
}

fn foo9() {
    let (a, b) = (2, 3);
    let sum = math2("sum");
    let product = math2("product");
    let div = math2("div");
    println!("{}", sum(a, b));
    println!("{}", product(a, b));
    println!("{}", div(a, b));
}

fn counter() -> fn(i32) -> i32 {
    fn inc(n: i32) -> i32 {
        n + 1
    }
    inc
}

fn foo10() {
    let f = counter();
    println!("{}", f(5));
}

fn counter2(i: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |n: i32| n + i)
}

fn foo11() {
    let f = counter2(2);
    println!("{}", f(7));
}

fn counter3(i: i32) -> impl Fn(i32) -> i32 {
    move |n: i32| n + i
}

fn foo12() {
    let f = counter3(2);
    println!("{}", f(6));
}

fn foo13() {
    // closure underlying: struct
    let c1 = || println!("i'm a closure");
    c1();
}

fn foo14() {
    let s = "hello".to_string();
    println!("{:p}", s.as_ptr());
    let c = || s; // value(s) moved into closure
    let r = c(); // value(s) moved to r
    println!("{}", r);
    println!("{:p}", r.as_ptr());
    // c();
    // c: FnOnce
}

fn foo15() {
    let mut x = 1;
    let mut inc = || x += 1;
    inc();
    inc();
    println!("{}", x);
}

fn foo16() {
    let a = [1, 2, 3];
    let mut iter = a.iter().map(|x| x * 2);
    println!("{}", iter.next().unwrap());
    println!("{}", iter.next().unwrap());
    println!("{}", iter.next().unwrap());
    println!("{:?}", iter.next());
}

fn foo17() {
    let arr1 = [1, 2, 3, 4, 5];
    let c1 = arr1.iter().map(|x| x * 2).collect::<Vec<i32>>();
    println!("{:?}", c1);
    let arr2 = ["1", "2", "3", "h"];
    let c2 = arr2
        .iter()
        .filter_map(|x| x.parse().ok())
        .collect::<Vec<i32>>();
    println!("{:?}", c2); // x.parse().ok(): Result.ok()
    let arr3 = ['a', 'b', 'c'];
    for (idx, val) in arr3.iter().enumerate() {
        println!("idx: {}, val: {}", idx, val);
    }
}

fn foo18() {
    let a = [1, 2, 3];
    let b = a.iter().any(|x| *x == 2);
    println!("{}", b);
    let s = a.iter().fold(0, |acc, x| acc + x);
    println!("{}", s);
}

#[derive(Debug, Clone, Copy)]
struct Book<'a> {
    name: &'a str,
    isbn: i32,
    version: i32,
}

fn foo19() {
    let book = Book {
        name: "Rust编程之道",
        isbn: 20181212,
        version: 1,
    };
    let book2 = Book { version: 2, ..book };
    println!("{:?}", book);
    println!("{:?}", book2);
}

fn main() {
    foo();
    foo2();
    foo3();
    foo4();
    foo5();
    foo6();
    foo7();
    foo8();
    foo9();
    foo10();
    foo11();
    foo12();
    foo13();
    foo14();
    foo15();
    foo16();
    foo17();
    foo18();
    foo19();
    foo20();
    foo21();
}

fn foo20() {
    let s = "\x1B[31;43mhello\x1B[0m";
    println!("{}", s);
    let s2 = "rust";
    let mut res = String::from("\x1B[");
    let bg_color = 31.to_string();
    let fg_color = 43.to_string();
    res.push_str(&bg_color);
    res.push_str(";");
    res.push_str(&fg_color);
    res.push_str("m");
    res.push_str(s2);
    res.push_str("\x1B[0m");
    println!("{}", res);
}

#[allow(dead_code)]
fn f() {
    println!("1");
}

// function shadow
fn foo() {
    f();
    {
        f();
        fn f() {
            println!("3")
        }
    }
    f();
    fn f() {
        println!("2");
    }
}

struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

struct CircleBuilder {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    fn new() -> CircleBuilder {
        CircleBuilder {
            x: 0.0,
            y: 0.0,
            radius: 1.0,
        }
    }
}

impl CircleBuilder {
    fn x(&mut self, coordinate: f64) -> &mut CircleBuilder {
        self.x = coordinate;
        self
    }

    fn y(&mut self, coordinate: f64) -> &mut CircleBuilder {
        self.y = coordinate;
        self
    }

    fn radius(&mut self, radius: f64) -> &mut CircleBuilder {
        self.radius = radius;
        self
    }

    fn build(&self) -> Circle {
        Circle {
            x: self.x,
            y: self.y,
            radius: self.radius,
        }
    }
}

fn foo21() {
    let c = Circle::new().x(1.0).y(2.0).radius(2.0).build();
    println!("{}", c.x);
    println!("{}", c.y);
    println!("{:.6}", c.area());
}