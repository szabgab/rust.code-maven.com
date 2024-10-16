fn main() {
    tools::use_struct();

    let p = tools::get_struct();
    println!("after get_struct: {:#?}", p);
    assert_eq!(p.fname, "Public");
    //println!("Hello {}", p.lname); // private field
}

mod tools {
    #[allow(dead_code)]
    #[derive(Debug)]
    pub struct Person {
        pub fname: String,
        lname: String,
    }

    pub fn use_struct() {
        let p = Person {
            fname: String::from("Foo"),
            lname: String::from("Bar"),
        };

        println!("in use_struct: {:#?}", p);
        assert_eq!(p.fname, "Foo");
        assert_eq!(p.lname, "Bar");
    }

    pub fn get_struct() -> Person {
        #[allow(clippy::let_and_return)]
        let p = Person {
            fname: String::from("Public"),
            lname: String::from("User"),
        };
        p
    }
}
