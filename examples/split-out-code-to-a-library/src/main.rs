mod other;

fn main() {
    hello_in_main_rs();

    split_out_code_to_a_library::hello_in_lib_rs();

    other::hello_in_other_rs();
    crate::other::hello_in_other_rs();
}


fn hello_in_main_rs() {
    println!("Hello in main rs");
}
