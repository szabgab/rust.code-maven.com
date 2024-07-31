fn main() {
    let numbers = vec![3, 5, 7];
    let ptr_1 = numbers.as_ptr() as *mut i32;
    println!("{:?}", numbers);
    //println!("{ptr_1:?}");
    println!("{ptr_1:p}");
    let mut ptr_1 = ptr_1 as usize;
    ptr_1 += 4;
    let nums = unsafe {
        Vec::from_raw_parts(ptr_1 as *mut i32, numbers.len() - 1, numbers.capacity() - 1)
    };
    println!("{:?}", nums);

}
