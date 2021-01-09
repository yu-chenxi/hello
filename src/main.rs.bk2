fn prime(n: u32) -> bool {
    let rt = (n as f64).sqrt() as u32;
    for i in 2..=rt {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn prime_n(n: u32) {
    // prime: 6n + 1 or 6n + 5
    let mut v = vec![2, 3, 5];
    for i in (7..=n).step_by(6) {
        if prime(i) {
            v.push(i);
        }

        if i + 4 <= n && prime(i + 4) {
            v.push(i + 4);
        }
    }

    const LN: usize = 10;
    let n = v.len();
    let prev_n = n / LN * LN;
    for i in (0..prev_n).step_by(LN) {
        println!("{:?}", &v[i..i + LN]);
    }

    println!("{:?}", &v[prev_n..]);
}

mod mymod;
fn main() {
    let mut args = std::env::args();
    let n = args.nth(1).expect("cargo run <num>");
    prime_n(n.parse::<u32>().expect("<num> must be u32 type!"));
    let v = [-10, 34, -91, 22];
    println!("{}", mymod::sum(&v));
}
