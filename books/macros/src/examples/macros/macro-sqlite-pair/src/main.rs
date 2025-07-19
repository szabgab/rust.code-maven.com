macro_rules! pair(
    ($one:expr, $two:expr) => (($one, Some($two)));
);

fn main() {
    let a = pair!("id", "1");
    assert_eq!(a, ("id", Some("1")));
}
