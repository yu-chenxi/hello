use std::ops::AddAssign;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Pt {
    x: i32,
    y: i32,
}

impl AddAssign for Pt {
    fn add_assign(&mut self, rhs: Self) {
        // todo!()
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Default for Pt {
    fn default() -> Self {
        // todo!()
        Pt { x: -1, y: -1 }
    }
}

fn main() {
    let mut pt1 = Pt { x: 1, y: 2 };
    let pt2 = Pt::default();
    pt1 += pt2;
    println!("pt1 = {:?}, pt2 = {:?}", pt1, pt2);
    foo();
    bar();
    // let b: bool = rand::random();
    // println!("b = {}", b);
}

fn foo() {
    // debug_assert_eq!(Pt { x: -1, y: 0 }, Pt::default()); // not be useful in release mode
}

fn bar() {
    // type MyRes = Result<i32, String>;
    // let x: MyRes = Err(String::from("a error"));
    // x.unwrap();
    // x.expect("expect i32 type");
    // assert_eq!(atcual, expected)
    // let x: MyRes = Ok(13);
    // x.unwrap_err(); // panic if Ok
}