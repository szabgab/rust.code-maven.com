#[allow(clippy::let_unit_value, clippy::unit_cmp)]

fn main() {
    let res = hello_world();
    assert_eq!(res, ());
    println!("{:?}", res);
}

fn hello_world() {
    println!("Hello, world!");
}
