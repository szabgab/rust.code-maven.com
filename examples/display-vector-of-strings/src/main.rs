use std::fmt;



// Try to implement Display for the Vec<String>
// impl fmt::Display for Vec<String> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}", "Something comes here")
//     }
// }

// Try to use a type-alias:
// type Words = Vec<String>;
// impl fmt::Display for Words {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}", "Something comes here")
//     }
// }


struct Words(Vec<String>);

impl fmt::Display for Words {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.join(","))
    }
}


fn main() {
    // plain vector
    //let animals: Vec<String> = vec![String::from("snake"), String::from("camel"), String::from("crab")];
    //println!("{}", animals);
    //println!("{:?}", animals);
    //println!("{:#?}", animals);

    // type alias
    //let animals: Words = vec![String::from("snake"), String::from("camel"), String::from("crab")];
    //println!("{:?}", animals);

    // one-element struct
    let animals: Words = Words(vec![String::from("snake"), String::from("camel"), String::from("crab")]);
    println!("{}", animals);
    println!("{}", animals.0[0]);
    println!("{}", animals.0[1]);
}
