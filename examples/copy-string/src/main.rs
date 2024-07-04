macro_rules! prt {
    ($name: expr) => {
        println!("ptr: {:?}", $name.as_ptr());
    };
}

fn main() {

    try_str();
    println!();
    try_string();
    println!();
    try_string_c();
    println!();
}


fn try_str() {
    let var_str = "hello";
    prt!(var_str);
    let assigned_str = var_str;
    prt!(assigned_str);
    prt!(var_str);
}

fn try_string() {
    let var_string = String::from("hello");
    prt!(var_string);
    let assigned_string = var_string;
    prt!(assigned_string);
    // prt!(var_string); // borrow of moved value: `var_string`
}

fn try_string_c() {
    let var_string = String::from("hello");
    prt!(var_string);
    let assigned_string = var_string.clone();
    //let assigned_string = var_string.to_owned();
    //let assigned_string = var_string.to_string();
    prt!(assigned_string);
    prt!(var_string);
}
