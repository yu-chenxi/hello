mod mymod;

// fat pointer
// attention: &mut [i32] != &mut Vec<i32>
// former: can not change size
// latter: can change size
fn rev(v: &mut [i32]) {
    let mut i = 0;
    let mut j = v.len() - 1;
    while i < j {
        let t = v[i];
        v[i] = v[j];
        v[j] = t;
        i += 1;
        j -= 1;
    }
}

use std::thread::sleep;
use std::time::Duration;

#[allow(dead_code)]
fn print_forever() -> ! {
    loop {
        println!("hello!");
        sleep(Duration::from_millis(500));
    }
}

trait Inc<T> {
    // self type is T
    fn inc(self) -> T;
}

impl Inc<i32> for i32 {
    fn inc(self) -> i32 {
        self + 1
    }
}

impl Inc<f64> for f64 {
    fn inc(self) -> f64 {
        self + 1.0
    }
}

fn main() {
    assert_eq!(mymod::is_match(&"(xx((hello))xxx)"), true);
    assert_eq!(mymod::is_match(&"(t)est("), false);
    assert_eq!(mymod::is_match(&"(((x))"), false);
    let s = "Hello Rust";
    let ptr = s.as_ptr();
    let len = s.len();
    println!("{:p}", ptr);
    println!("{:?}", len); // not include '\0'
    let mut v = [2, 5, 11];
    rev(&mut v);
    println!("{:?}", v);
    println!("{}", std::mem::size_of::<&mut Vec<i32>>());
    /*
    [ptr]
    [len]
    */
    println!("{}", std::mem::size_of::<&mut [i32]>());
    /*
    ->
        [ptr]
        [len]
        [cap]
    */
    //print_forever();
    let pt1 = Pt { x: 1, y: 2 };
    let pt2 = Pt { x: 3.3, y: 4.4 };
    println!("{:?}", pt1);
    println!("{:?}", pt2);
    let duck = Duck;
    println!("{}", fly_static(duck));
    let (x, y): (i32, f64) = (2, 3.3);
    println!("{}", x.inc());
    println!("{}", y.inc());
    let s1 = "hello";
    let s2 = " world!";
    println!("{}", s1.to_string() + s2); // operator overload
}

#[derive(Debug)]
struct Pt<T> {
    x: T,
    y: T,
}

impl<T> Pt<T> {}

struct Duck;
trait Fly {
    fn fly(&self) -> bool;
}

// not duck typing
impl Fly for Duck {
    fn fly(&self) -> bool {
        true
    }
}

fn fly_static<T: Fly>(t: T) -> bool {
    t.fly()
}

/*
1. qemu-img create -f raw <vd> <size>
2. -append "root=/dev/sda init=/linuxrc ip=dhcp nokaslr console=ttyS0"
-drive format=raw,file=<vd>
3. INSTALL_MOD_PATH=<vd-mp> make modules_install
4. /etc/init.d/rcS
mount -t proc none /proc
mount -t sysfs none /sys
*/

/*
static lang: check type in compilation-time
dynamic lang: check type in runtime
strong type: go(explicitly type conversion)
weak type: C(implicitly type conversion)
type safe
polymorphism:
static poly(compilation-time): zero cost
dynamic poly(runtime):
rust poly: generic type + trait
Option<T>: T or null
Result<T, E>: T or error
sized type: i32, &str, &mut [i32], &mut Vec<i32>
dynamic sized type
zero sized type: struct UnitStruct
trait + generic
*/
