#[cfg(test)]
mod my {
    use super::*;

    macro_rules! make_test {
        ($n:expr) => {
            (($n as f64).sqrt(), $n)
        };
    }
    #[test]
    fn test_my_sqrt() {
        let tests = vec![
            make_test!(2.0),
            make_test!(13.0),
            make_test!(65537.0),
            make_test!((2 << 28 - 5) as f64),
        ];
        for (act, x) in tests {
            let rt = my_sqrt(x);
            let diff = (act - rt).abs();
            assert!(diff < 1e-6);
        }
        // Err("unknown error!")
    }
}

fn my_sqrt(x: f64) -> f64 {
    const N: u32 = 16;
    let mut rt = x / 2.0;
    for _ in 0..N {
        rt = (rt + x / rt) / 2.0;
    }
    rt
}

fn foo() {
    println!("{:.3}", my_sqrt(2.0));
    println!("{:.3}", my_sqrt(17.0));
    println!("{:.3}", my_sqrt(65537.0));
}

fn main() {
    foo();
}

// alt-up: move line up
// alt-down: move line down
// shift-alt-up: copy line up
// ctrl-shift-\: {} ()