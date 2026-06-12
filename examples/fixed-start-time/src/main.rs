use std::sync::LazyLock;
use std::time::SystemTime;

static START_TIME: LazyLock<SystemTime> = LazyLock::new(SystemTime::now);

fn main() {
    print_start_time();
    print_start_time();

}

fn print_start_time() {
    println!("{:?}", *START_TIME);
}
