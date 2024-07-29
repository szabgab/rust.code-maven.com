macro_rules! macro_with_literal {
	($var:literal) => {
        println!("{:?}", $var)
    }
}

fn main() {
    macro_with_literal!("hello");
    macro_with_literal!(42);
    macro_with_literal!(true);
    //macro_with_literal!(x);
    //macro_with_literal!(String::from("hello"));
}

