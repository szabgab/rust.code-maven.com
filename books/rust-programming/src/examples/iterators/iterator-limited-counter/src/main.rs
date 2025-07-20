fn main() {
    let mut cnt = tools::Counter::new(5);
    //let cnt = tools::Counter::new_till42();
    // let cnt = tools::Counter {
    //     current: 3,
    //     limit: 7,
    // };
    println!("{:?}", &cnt);
    // for x in cnt {
    //     println!("{}", x);
    // }

    loop {
        match cnt.next() {
            Some(val) => println!("{val}"),
            None => {
                println!("done");
                break;
            }
        }
    }
}

mod tools {
    #[derive(Debug)]
    pub struct Counter {
        current: u32,
        limit: u32,
    }

    impl Counter {
        pub fn new(limit: u32) -> Counter {
            Counter { current: 0, limit }
        }
        // pub fn new_till42() -> Counter {
        //     Counter { current: 0, limit: 42 }
        // }
    }

    impl Iterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            self.current += 1;
            if self.current > self.limit {
                return None;
            }
            Some(self.current)
        }
    }
}
