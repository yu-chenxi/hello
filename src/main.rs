fn sort() {
    const N: usize = 1000000;
    let mut v = Vec::with_capacity(N);
    for _ in 0..N {
        v.push(rand::random::<i32>() % (N as i32));
    }
    v.sort();
    print!("{:?}", &v[..10]);
}

fn main() {
    sort();
}
