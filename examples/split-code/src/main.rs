use app::tools::in_tools;

fn main() {
    println!("in main function");
    in_main();
    app::in_lib();
    app::helper::in_helper();
    app::in_imported_helper();

    in_tools();
}


fn in_main() {
    println!("in src/main.rs");
}
