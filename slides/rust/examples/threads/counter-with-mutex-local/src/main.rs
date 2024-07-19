fn main() {
    let limit = 1_000_000;

    let threads = 10;

    let result = count_with_mutex(threads, limit);

    println!("{}", result);
    assert_eq!(result, limit * threads);
}

fn count_with_mutex(threads: i32, limit: i32) -> i32 {
    let counter = std::sync::Mutex::new(0);

    std::thread::scope(|scope| {
        for _ in 0..threads {
            scope.spawn(|| {
                println!("Start {:?}", std::thread::current().id());
                let mut my_counter = 0;
                for _ in 0..limit {
                    my_counter += 1;
                }
                let mut guard = counter.lock().unwrap();
                *guard += my_counter;
            });
        }
    });

    counter.into_inner().unwrap()
}

