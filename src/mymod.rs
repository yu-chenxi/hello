#[allow(dead_code)]
pub fn sum(v: &[i32]) -> i32 {
    let mut s = 0;
    for x in v.iter() {
        s += x;
    }
    s
}

use std::collections::VecDeque;
pub fn is_match(s: &str) -> bool {
    let mut st = VecDeque::new(); // type: VecDeque<_>
    for c in s.chars() {
        match c {
            '(' => st.push_front(c),
            ')' => {
                if st.is_empty() || '(' != st.pop_front().unwrap() {
                    return false;
                }
            }
            _ => (),
        };
    }
    st.is_empty()
}
