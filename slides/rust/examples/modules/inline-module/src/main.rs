fn main() {
    println!("in main");

    // crate::tools::private_helper(); // private function

    tools::public_helper();
    crate::tools::public_helper();
}

mod tools {
    fn private_helper() {
        println!("in tools::private_helper");
    }

    pub fn public_helper() {
        println!("in tools::public_helper");

        private_helper();
    }
}
