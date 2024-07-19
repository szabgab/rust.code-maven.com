use threaded_map::ThreadedMappable;

fn main() {
    let items = vec![1, 2, 3, 4, 5, 6];
    let target = items.iter().map(|num| double(*num)).collect::<Vec<_>>();
    println!("{:?}", target);

    let results = items
        .into_iter()
        .threaded_map(double, None)
        .collect::<Vec<_>>();

    println!("{:?}", results);
    assert_eq!(results, target);
}

fn double(n: i32) -> i32 {
    println!("{:?}", std::thread::current().id());
    n * 2
}
