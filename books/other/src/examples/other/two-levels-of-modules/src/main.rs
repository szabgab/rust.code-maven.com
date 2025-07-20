#![allow(dead_code)]

fn main() {
    println!("in main");
    //house::live();
    //crate::house::live();

    // kitchen is private
    //house::kitchen::cook();
    //crate::house::kitchen::cook();

    //crate::house::bathroom::shower();
    //house::bathroom::shower();

    house::bathroom::shaving();
}

mod house {
    pub fn live() {
        println!("live");
        kitchen::cook();
        bathroom::shower();
    }

    mod kitchen {
        pub fn cook() {
            println!("cook");
            peal_potatoes();
        }
        fn peal_potatoes() {
            println!("peal_potatoes");
        }

        pub fn doing_dishes() {
            println!("doing_dishes");
        }
    }

    pub mod bathroom {
        pub fn shower() {
            println!("shower");
            use_soap()
        }

        fn use_soap() {
            println!("use_soap");
        }

        pub fn shaving() {
            println!("shaving");
            crate::house::kitchen::cook();
            super::kitchen::cook();
        }
    }
}
