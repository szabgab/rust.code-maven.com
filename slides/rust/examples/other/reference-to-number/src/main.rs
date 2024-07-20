fn main() {
//    copy_and_change();
//    reference_is_borrow();
    reference_in_scope_is_borrow();

}



fn copy_and_change() {
    let mut a = 23;
    let b = 23;

    a += 1;

    println!("{a} {:p}", &a);
    println!("{b} {:p}", &b);
}

fn reference_is_borrow() {
    let mut a = 23;
    a += 1;
    
    let b = &a;

    //a += 1; // cannot assign to `a` because it is borrowed


    println!("{a} {:p}", &a);
    println!("{b} {:p}", &b);
}


fn reference_in_scope_is_borrow() {
    let mut a = 23;
    a += 1;

    let b = &a;
    println!("{b} {:p}", &b);  // the lifetime of the reference ends here

    a += 1; // cannot assign to `a` because it is borrowed

    println!("{a} {:p}", &a);
}
