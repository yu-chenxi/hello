fn main() {
    let v: Vec<i32> = vec![1, 2, 3];
    println!("{:?}", v);
    let s: i32 = v.iter().sum();
    println!("{}", s);
    let v2: Vec<i32> = v.iter().map(|x| x * x).collect();
    println!("{:?}", v2);
    let s = v.iter().fold(1, |x, y| 2 * x + y);
    // f(-1) = 1
    // f(x+1) = 2 * f(x) + v[x+1]
    // f(0) = 2 * 1 + 1 = 3
    // f(1) = 2 * 3 + 2 = 8
    // f(2) = 2 * 8 + 3 = 19
    println!("{}", s);
    foo();
}

trait Fly {
    fn fly(&self);
}

struct Duck;
struct Pig;

impl Fly for Duck {
    fn fly(&self) {
        // todo!()
        println!("duck can fly.");
    }
}

impl Fly for Pig {
    fn fly(&self) {
        // todo!()
        println!("pig can't fly.");
    }
}

fn static_dispatch<T>(t: &T)
where
    T: Fly,
{
    t.fly();
}

fn dynamic_dispatch(t: &dyn Fly) {
    t.fly();
}

fn foo() {
    let duck = Duck;
    let pig = Pig;
    static_dispatch(&duck);
    static_dispatch(&pig);
    dynamic_dispatch(&duck); // &dyn Fly = T* + vtable
    dynamic_dispatch(&pig);
}