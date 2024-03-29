use std::io;
use sysinfo::System;

fn main() {
    allocate();
    //allocate();
}

fn allocate() {

    let filler = "0123456789".repeat(10);
    let size = 200_000_000;
    println!(
        "Size: {size} filler: {} total: {}",
        filler.len(),
        size * filler.len()
    );

    let used_before = show_memory();

    println!("Before");
    let mut text = String::with_capacity(size * filler.len());
    for _ in 0..size {
        text.push_str(&filler);
    }
    println!("Allocated");
    let used_allocated = show_memory();
    println!(
        "Memory used since start of the program: {}",
        used_allocated - used_before
    );

    //wait_for_enter();

    println!("After");
    show_memory();
}


fn total_memory() -> u64 {
    let mut sys = System::new_all();
    sys.refresh_all();
    sys.total_memory()
}

fn used_memory() -> u64 {
    let mut sys = System::new_all();
    sys.refresh_all();
    sys.used_memory()
}


fn wait_for_enter() {
    println!("Press ENTER to continue");
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer).unwrap();
}

fn show_memory() -> u64 {
    let mut sys = System::new_all();
    sys.refresh_all();

    println!("total memory: {} bytes", sys.total_memory());
    println!("used memory : {} bytes", sys.used_memory());
    println!("total swap  : {} bytes", sys.total_swap());
    println!("used swap   : {} bytes", sys.used_swap());
    sys.used_memory()
}
