#![allow(unused_macros, unused_imports)]

macro_rules! prt {
    ($var: expr) => {
        println!("{:?}", $var);
    };
}
pub(crate) use prt;
