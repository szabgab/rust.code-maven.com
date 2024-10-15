// #![allow(dead_code)]

// #[cfg(test)]
// use mockall::{automock, predicate::*};
// #[cfg_attr(test, automock)]
//  trait MyTrait {
//     fn foo(&self, x: u32) -> u32;
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn mytest() {
//         let mut mock = MockMyTrait::new();
//         mock.expect_foo()
//             .with(eq(4))
//             .times(2)
//             .returning(|_x| 5);
// //            .returning(|x| x + 1);
//         assert_eq!(5, mock.foo(4));
//         assert_eq!(5, mock.foo(4));
//         // assert_eq!(12, mock.foo(10));
//     }
// }



// // -------------------------------------------
#![allow(dead_code)]


pub trait Area {
    fn area(&self) -> u32;
}


#[cfg(test)]
use mockall::{automock, predicate::*};
#[cfg_attr(test, automock)]
pub trait Long {
    fn long(&self);
}


struct Rectangle {
    length: u32,
    width: u32,
}

impl Area for Rectangle {
    fn area(&self) -> u32 {
        self.long();
        self.width * self.length
    }
}

impl Long for Rectangle {
    fn long(&self) {
        std::thread::sleep(std::time::Duration::from_secs(20)); 
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let r = Rectangle { length: 2, width: 3};
        // assert_eq!(r.area(), 6);

        let mut mock = MockLong::new();
        mock.expect_long()
            .with()
//            .times(1)
            .returning(|| ());

        let r = Rectangle { length: 2, width: 3};
        assert_eq!(r.area(), 6);

    }
}




// ------------------------------------------------
// struct Rectangle {
//     length: u32,
//     width: u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.long();
//         self.width * self.length
//     }
// }

// impl Rectangle {
//     fn long(&self) {
//         std::thread::sleep(std::time::Duration::from_secs(20)); 
//     }
// }


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let r = Rectangle { length: 2, width: 3};
//         assert_eq!(r.area(), 6);
//     }
// }

// --------------------------------------

// pub fn add(left: u64, right: u64) -> u64 {
//     long();
//     left + right
// }

// pub fn long() {
//     std::thread::sleep(std::time::Duration::from_secs(20));
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
