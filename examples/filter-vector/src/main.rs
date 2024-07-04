use sysinfo::System;
use thousands::Separable;


#[derive(Debug)]
#[allow(dead_code)]
struct Thing {
    name: String,
    number: i64,
}


fn main() {
    used_memory("before");
    let things = (1..=100_000_000).map(|number| Thing {
        name: format!("thing {number}"),
        number: number,
    }).collect::<Vec<Thing>>();
    used_memory("created");
    //println!("{:#?}", things);
    println!("things:   {:>10}", things.len());
    //let middle = things.len() / 2;
    //let filtered = filter_by_cond(&things, |thing| thing.number <= middle);
    // Error: found clousure

    let filtered = filter_by_cond(&things, |thing| thing.number <= 10_000_000-10);
    println!("filtered: {:>10}", filtered);
    used_memory("after 1");

    let filtered = filter_by_cond(&things, |thing| thing.number > 10);
    println!("filtered: {:>10}", filtered);
    used_memory("after 2");

    // let strings = (1..=10).map(|number| format!("heskfskjfhskgakdgdkshbgdlllo {number}")).collect::<Vec<String>>();
    // println!("total: {}", strings.len());
    // println!("{}", mem::size_of_val(&5i32));
    // println!("{}", mem::size_of_val(&strings[0]));

}

fn filter_by_cond(
    things: &[Thing],
    cond: fn(&&Thing) -> bool,
) -> usize {
    //let filtered = things.iter().filter(cond).cloned().collect::<Vec<Thing>>();
    let filtered = things.iter().filter(cond).collect::<Vec<&Thing>>();
    used_memory("in filter");

    filtered.len()
}

fn used_memory(title: &str) -> u64 {
    let mut sys = System::new_all();
    sys.refresh_all();

    println!(
        "used memory: ({:15}) {:>15} bytes free memory: {:>15}",
        title,
        sys.used_memory().separate_with_commas(),
        sys.free_memory().separate_with_commas()
    );

    sys.used_memory()
}

