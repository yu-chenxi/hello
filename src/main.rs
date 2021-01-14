fn f(mut s: String) -> String {
    s.push_str("world!");
    s
}

fn main() {
    let mut s = String::from("hello ");
    s = f(s);
    println!("s = {}", s);
}