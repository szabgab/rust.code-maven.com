use sysinfo::System;
use thousands::Separable;

fn main() {
    show_memory();
}

fn show_memory() {
    let mut sys = System::new_all();
    sys.refresh_all();

    println!(
        "total memory: {:>15} bytes",
        sys.total_memory().separate_with_commas()
    );
    println!(
        "used memory : {:>15} bytes",
        sys.used_memory().separate_with_commas()
    );
    println!(
        "total swap  : {:>15} bytes",
        sys.total_swap().separate_with_commas()
    );
    println!(
        "used swap   : {:>15} bytes",
        sys.used_swap().separate_with_commas()
    );
}
