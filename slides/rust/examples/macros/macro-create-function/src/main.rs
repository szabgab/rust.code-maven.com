macro_rules! define_function {
    ($var:ident) => {
        fn $var() {
            println!("hi");
        }
    };
}

fn main() {
    define_function!(hi);
    hi();
}
