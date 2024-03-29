
use sysinfo::System;
use thousands::Separable;

fn main() {
    println!("Total memory: {}",  total_memory().separate_with_commas());
    let filler = "0123456789".repeat(10);
    let size = 200_000_000;

    println!(
        "Size: {} filler: {} total: {}",
        size.separate_with_commas(),
        filler.len(),
        (size * filler.len()).separate_with_commas()
    );

    let used_before = show_memory();
    println!("Used memory before: {}",  used_before.separate_with_commas());

    allocate(size, &filler);

    println!(
        "Memory used since start of the program: {}",
        (used_allocated - used_before).separate_with_commas()
    );

    let used_after = show_memory();
    println!("Used memory after: {}",  used_after.separate_with_commas());

    //allocate(size, &filler);
}

fn allocate(size: usize, filler: &str) {

    let mut text = String::with_capacity(size * filler.len());
    for _ in 0..size {
        text.push_str(filler);
    }

    let used_allocated = show_memory();
    println!("Used memory allocated: {}",  used_allocated.separate_with_commas());

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



