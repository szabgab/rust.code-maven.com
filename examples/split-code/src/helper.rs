pub fn in_helper() {
    println!("in_helper");
}

pub fn in_imported_helper() {
    println!("in_imported_helper");

    crate::in_lib()
}
