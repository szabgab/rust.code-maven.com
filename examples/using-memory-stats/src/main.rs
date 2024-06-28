use memory_stats::memory_stats;
use thousands::Separable;

fn main() {
    show_mem();

    println!("Bytes            Physical memory   Virtual memory  ");
    check_mem(10000);
    check_mem(100000);
    check_mem(1000000);
    check_mem(10000000);
    check_mem(100000000);
    check_mem(1000000000);
    check_mem(10000000000);
}

fn check_mem(bytes: usize) {
    let before = memory_stats().unwrap();
    let _text = String::from("x".repeat(bytes));
    let after = memory_stats().unwrap();

    let physical_mem = after.physical_mem - before.physical_mem;
    let virtual_mem = after.virtual_mem - before.virtual_mem;
    println!(
        "{:>15} {:>15} {:>15}",
        bytes.separate_with_commas(),
        physical_mem.separate_with_commas(),
        virtual_mem.separate_with_commas()
    )
}

fn show_mem() {
    if let Some(usage) = memory_stats() {
        println!(
            "Physical memory usage: {:>15}",
            usage.physical_mem.separate_with_commas()
        );
        println!(
            "Virtual memory usage:  {:>15}",
            usage.virtual_mem.separate_with_commas()
        );
    } else {
        println!("Couldn't get the current memory usage :(");
    }
}
