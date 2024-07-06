fn main() {
    let mut joe = Person {name: String::from("Joe"), partner: None};
    #[allow(unused_mut)]
    let mut jane = Person {name: String::from("Jane"), partner: None};
    dbg!(&joe);
    dbg!(&jane);
    joe.partner = Some(&jane);
    //jane.partner = Some(&joe);
    dbg!(&joe);
    dbg!(&jane);

}

#[derive(Debug)]
#[allow(dead_code)]
struct Person<'a> {
    name: String,
    partner: Option<&'a Person<'a>>,
}

