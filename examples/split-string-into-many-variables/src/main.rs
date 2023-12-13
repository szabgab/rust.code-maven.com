fn main() {
    for text in [
        "Foo:Bar:42",
        "Foo:Bar:",
        "Foo:Bar:42:garbage",
        "Foo:Bar",
    ] {
        println!("str: {text}");
        splitn_and_collect(text);
        splitn_collect_iter(text);
        splitn_and_next(text);
    }
}


// The call to `next` will panic! if there are not enough parts
fn splitn_and_next(text: &str) {
    println!("str: {text}");
    let mut parts = text.splitn(3, ':');

    let fname = parts.next().unwrap();
    let lname = parts.next().unwrap();
    let value = parts.next().unwrap();
    println!("fname: {fname}");
    println!("lname: {lname}");
    println!("value: {value}");
    println!();
}


fn splitn_and_collect(text: &str) {
    let parts = text.splitn(3, ':').collect::<Vec<&str>>();
    if parts.len() != 3 {
        eprintln!("Not enough parts");
        return;
    }

    let (fname, lname, value) = (parts[0], parts[1], parts[2]);
    println!("fname: {fname}");
    println!("lname: {lname}");
    println!("value: {value}");
    println!();
}


fn splitn_collect_iter(text: &str) {
    use itertools::Itertools;
    let parts = text.splitn(3, ':').collect::<Vec<&str>>();
    if parts.len() != 3 {
        eprintln!("Not enough parts");
        return;
    }

    if let Some((fname, lname, value)) = parts.into_iter().tuples().next() {
        println!("fname: {fname}");
        println!("lname: {lname}");
        println!("value: {value}");
    
    }
    println!();
}




