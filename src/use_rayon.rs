// use rayon::prelude::*;

fn sum2(input: &[i32]) -> i32 {
    // input.par_iter().map(|i| i * i).sum()
    0
}

pub fn foo() {
    // use rayon
    let v = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
    let s = sum2(&v);
    println!("s = {}", s);
}
