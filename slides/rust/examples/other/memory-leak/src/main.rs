#![allow(unused_variables)]

#[allow(dead_code)]
struct Thing {
    data: String,
    other: Option<Box<Thing>>
}


fn main() {
    alloc();
}

fn alloc() {
    let mut a = Thing {
        data: String::from("abc"),
        other: None,
    };

    let mut b = Thing {
        data: String::from("abc"),
        other: None,
    };
    
    a.other = Some(Box::new(b));
    b.other = Some(Box::new(a));
}
