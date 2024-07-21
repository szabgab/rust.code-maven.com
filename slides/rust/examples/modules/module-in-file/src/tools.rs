fn private_helper() {
    println!("in tools::private_helper");
}


pub fn public_helper() {
    println!("in tools::public_helper");

    private_helper();
}
