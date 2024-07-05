fn main() {
    println!("main");
    in_main_rs();
    // colors::red(); // private function

    colors::red();
    crate::colors::red();

    colors::deep::in_deep();


    // colors::blue();
    // // colors::blue_helper(); // error[E0603]: function `blue_helper` is private
    // colors::dark::blue();

    // use colors::dark;
    // dark::green();
}

fn in_main_rs() {
    println!("in_main_rs");
}

mod colors {
    // fn red() {
    //     println!("red");
    // }

    pub fn red() {
        println!("public red");
        deep::in_deep();
    }

    // pub fn blue() {
    //     println!("blue");
    //     blue_helper(); // can be called from here
    // }

    pub mod deep {
        pub fn in_deep() {
            println!("in_deep");
        }
    }

    // fn blue_helper() {
    //     println!("blue_helper");
    //     crate::in_main_rs();
    //     super::in_main_rs();
    // }

    // pub mod dark {
    //     pub fn blue() {
    //         println!("dark_blue");
    //         crate::in_main_rs();        // absolute path
    //         super::super::in_main_rs(); // relative path, probably not very good idea
    //         super::red();
    //     }
    //     pub fn green() {
    //         println!("dark_green");
    //     }
    // }

}

