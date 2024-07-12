
#[derive(Debug)]
#[allow(dead_code)]
struct Thing {
    data: String,
    next: Option<Box<Thing>>,
}


fn main() {
    let a = Thing {
        data: String::from("First"),
        next: None,
    };
    println!("{:#?}", a);


    let b = Thing {
        data: String::from("Second"),
        next: Some(Box::new(a)),
    };
    println!("{:#?}", b);

    let c = Thing {
        data: String::from("Third"),
        next: Some(Box::new(b)),
    };
    println!("{:#?}", c);

}
