use std::time::SystemTime;
use cached::macros::cached;

fn main() {
    let time = get_current_time();
    println!("Current time: {:?}", time);
    let time = get_current_time();
    println!("Current time: {:?}", time);


    let time = get_cached_time();
    println!("Cacehed time: {:?}", time);
    let time = get_cached_time();
    println!("Cacehed time: {:?}", time);
}

fn get_current_time() -> SystemTime {
    SystemTime::now()
}


#[cached]
fn get_cached_time() -> SystemTime {
    SystemTime::now()
}
