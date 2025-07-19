macro_rules! define_with_tt {
    ($var:tt) => {
        fn $var() {
            println!("hi");
        }
    };
}

macro_rules! define_with_ident {
    ($var:ident) => {
        fn $var() {
            println!("hi");
        }
    };
}

fn main() {
    define_with_tt!(hi_tt);
    define_with_ident!(hi_ident);
    hi_tt();
    hi_ident();

    global_tt();
    global_ident();
}

define_with_tt!(global_tt);
define_with_ident!(global_ident);
