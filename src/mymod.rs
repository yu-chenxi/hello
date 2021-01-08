pub fn sum(v: &[i32]) -> i32 {
    let mut s = 0;
    for x in v.iter() {
        s += x;
    }
    s
}
