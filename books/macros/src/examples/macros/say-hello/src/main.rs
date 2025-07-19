macro_rules! say_hello {
    ($name: expr) => {
        println!("Hello {}!", $name);
    };
}

fn main() {
    say_hello!("Foo");
    say_hello!("Bar");
    say_hello!("Foo Bar");

    // say_hello!("Foo", "Bar");
    // ^ no rules expected this token in macro call

    // say_hello!();
    // ^^^^^^^^^^^^ missing tokens in macro arguments
}
