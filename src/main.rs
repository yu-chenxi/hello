mod mymod;

fn main() {
    assert_eq!(mymod::is_match(&"(xx((hello))xxx)"), true);
    assert_eq!(mymod::is_match(&"(t)est("), false);
    assert_eq!(mymod::is_match(&"(((x))"), false);
}