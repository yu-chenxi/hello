use std::io::{BufWriter, Read, Write};
use std::{fs::File, io::Error};

fn main() {
    foo();
    foo2();
    foo3("./test.txt").unwrap();
    foo4().unwrap();
}

fn foo4() -> Result<(), Error> {
    let file = File::create("./new.txt")?;
    let mut buf = BufWriter::new(file);
    write!(buf, "{} * {} = {}", 11, 13, 11 * 13)?;
    buf.flush()
}

fn foo3(file: &str) -> Result<(), Error> {
    let mut file = File::open(file)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    print!("content:\n{}", buf);
    Ok(())
}

fn foo() {
    let mut x = Box::new(3);
    *x += 1; // *(x.deref())
    println!("{}", x);
}

trait Anim {
    fn kind(&self) {
        println!("animal");
    }
}

#[derive(Debug)]
struct Dog(u64);
impl Anim for Dog {
    // @override
    fn kind(&self) {
        println!("dog");
    }
}

impl Drop for Dog {
    fn drop(&mut self) {
        println!("Dog type drop");
    }
}

fn dyn_disp(a: Box<dyn Anim>) {
    a.kind();
}

fn foo2() {
    let a: &dyn Anim;
    let dog = Dog(5);
    a = &dog;
    a.kind();
    let dog = Box::new(Dog(3));
    dyn_disp(dog); // moved
                   // println!("{:?}", dog);
}
