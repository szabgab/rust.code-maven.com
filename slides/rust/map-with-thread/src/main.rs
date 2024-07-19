use threaded_map::ThreadedMappable;

fn main() {
    let items = vec![1, 2, 3, 4, 5, 6];
    let target: Vec<_> = items.iter().map(i32::to_string).collect();

    let result: Vec<_> = items
        .into_iter()
        .parallel_map(|item| item.to_string(), None)
        .collect();

    assert_eq!(result, target);
}
