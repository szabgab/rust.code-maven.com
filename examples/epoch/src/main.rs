use std::time::{SystemTime, UNIX_EPOCH};
fn main() {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    println!("start:           {start:?}", );
    println!("since_the_epoch: {since_the_epoch:?}");
    println!("as_nanos:        {}", since_the_epoch.as_nanos());
    println!("as_micros:       {}", since_the_epoch.as_micros());
    println!("as_millis:       {}", since_the_epoch.as_millis());
    println!("as_secs:         {}", since_the_epoch.as_secs());
    println!("as_secs_f32:     {}", since_the_epoch.as_secs_f32());
    println!("as_secs_f64:     {}", since_the_epoch.as_secs_f64());
}

