fn main() {
    println!("in main");

    // crate::tools::private_helper(); // private function

    tools::public_helper();
    crate::tools::public_helper();
}

mod tools;
