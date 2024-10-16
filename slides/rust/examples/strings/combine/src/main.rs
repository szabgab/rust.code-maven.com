fn main() {
    let fname = String::from("Foo");
    let lname = String::from("Bar");
    println!("{fname}");
    println!("{lname}");

    let res = combine(&fname, &lname);

    println!("Combined name {res}");
    println!("{fname}");
    println!("{lname}");
}

fn combine(fname: &str, lname: &str) -> String {
    //format!("{fname} {lname}")
    fname.to_owned() + " " + lname
}
