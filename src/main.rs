#[derive(Debug, PartialEq)]
struct People {
    name: &'static str,
    gender: char,
}

impl People {
    fn new(name: &'static str, gender: char) -> Self {
        Self { name, gender }
    }

    fn name(&self) {
        println!("{}", self.name);
    }

    fn gender(&self) {
        match self.gender {
            'M' => println!("boy."),
            'F' => println!("girl."),
            _ => println!("unknown!"),
        };
    }
}

fn main() {
    let mut p = People::new("mike", 'F');
    p.name();
    p.gender();
    p.gender = 'X';
    p.gender();
    assert_eq!(
        p,
        People {
            name: "mike",
            gender: 'X'
        }
    );
    // let x: Option<i32> = None;
    // println!("{}", x.unwrap());
    println!("{}", { "hello" }); // { "hello" } -> "hello"
    println!("{:?}", {
        "hello";
    }); // { "hello"; } -> ()
    let s = String::from("rust");
    let s2 = { s };
    // println!("{}", s); // borrow of moved value
    println!("{}", s2);
    let x = {};
    assert_eq!(x, ()); // () void type
    let x1 = 2;
    let x2 = 3; // statement
    let m = if x1 > x2 {
        // expression(true/false)
        x1 // i32
    } else {
        // x2; // ()
        x2 // expression
    };
    println!("{}", m);
    println!("{}", std::mem::size_of::<()>()); // size = 0
}

/*
tiny sshd:
1. server end:
mkfifo p
nc -l <port> < p | sh -i &> p

2. client end:
nc -N -w <timeout> <host> <port>
Ctrl-D to quit

port scan:
nc -zv -w <timeout> <host> <port>...
nc -zv -w 2 www.baidu.com 80 443
nc -zv -w 2 www.baidu.com 440-450

file transfer:
1. sender
nc -N <host> <port> < <file>

2. receiver
nc -l <host> > <file>

http get:
printf "GET / HTTP/1.1\r\n\r\n" | nc -N <host> <port> > <html-file>
*/
