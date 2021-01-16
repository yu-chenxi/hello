use std::panic;

fn _my_div(x: i32, y: i32) -> Result<i32, String> {
    if y != 0 {
        Ok(x / y)
    } else {
        Err(String::from("div 0!"))
    }
}

fn my_div() {
    let x = 3;
    let mut y = 1;
    println!("{}", _my_div(x, y).unwrap());
    y = 0;
    println!("{}", _my_div(x, y).unwrap());
}

fn main() {
    let res = panic::catch_unwind(|| {
        my_div();
    });

    if let Err(_) = res {}
    println!("catch the panic!");
}
