fn main() {
    tools::use_struct();

    let p = tools::get_struct();
    println!("Hello {:#?}", p);

    println!("Hello {}", p.fname);
    //println!("Hello {}", p.lname);
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

        println!("Hello {:#?}", p);
    }


    pub fn get_struct() -> Person {
        let p = Person {
            fname: String::from("Public"),
            lname: String::from("User"),
        };
        p
    }

}