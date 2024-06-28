// #[derive(Debug)]
// pub enum A {
//     B(B),
//     C(C)
// }

// #[derive(Debug)]
// pub struct B {
//     pub bar: usize
// }

// #[derive(Debug)]
// pub struct C {
//     pub bar2: usize
// }




// fn main() {
//     let b = B {
//         bar: 23
//     };
//     println!("{b:?}");


//     //let x = A::B(b);
//     let x = A::B( B { bar: 12} );
//     println!("{x:?}")
// }


#[derive(Debug)]
pub enum A {
    B(bool),
    C(String)
}

#[derive(Debug)]
#[allow(dead_code)]
struct Thing {
    readme: A,
}

fn main() {
    let x = A::B(true);
    println!("{x:?}");

    let y = A::C(String::from("hi"));
    println!("{y:?}");

    let a = Thing { readme: A::C(String::from("foo"))};
    println!("{a:?}");
}
