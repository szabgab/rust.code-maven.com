fn main() {
    let mut text = String::from("abcd");
    println!("{text}");
    let pointer: *mut String = &mut text as *mut String; // mutable raw pointer
    println!("pointer is: {:?}", pointer);
    println!("addr_of: {:?}", std::ptr::addr_of!(text));
    unsafe {
        // pointer.offset(count);
        println!("pointer+1 is: {:?}", pointer.offset(1));
        println!("pointer is: {}", *pointer);

        //println!("ptr is: {}", *ptr.offset(1));
    }

    println!("{text}");

    // let address = 0x012345usize;
    // let r = address as *const i32;

    // println!("{r:?}");

    //safe_increment();
    // unsafe_increment();
}

// fn safe_increment() {

//     let mut num = 5;

//     let r1 = &num;

//     num += 1;
    
//     println!("{num}");
//     println!("{r1}");
// }


// fn unsafe_increment() {

//     let mut num = 5;

//     let r1 = &num as *const i32; // immutable raw pointer
//     let r2 = &mut num as *mut i32; // mutable raw pointer
//     println!("r1 is: {:?}", r1);
//     println!("r2 is: {:?}", r2);
//     unsafe {
//         println!("r1 is: {}", *r1);
//         println!("r2 is: {}", *r2);
//         *r2 += 1;
    
//     }
//     println!("{num}");
// }
