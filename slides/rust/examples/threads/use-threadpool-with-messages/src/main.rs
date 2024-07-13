use threadpool::ThreadPool;
use std::sync::mpsc::channel;

fn main() {
    let numbers = (0..100).collect::<Vec<u32>>();
    //numbers.iter().map(|val| do_something(*val)).for_each(drop);

    
    let n_workers = 4;
    let pool = ThreadPool::new(n_workers);
    
    let (tx, rx) = channel();
    let count = numbers.len();
    for num in numbers {
        let tx = tx.clone();
        pool.execute(move|| {
            do_something(num);
            tx.send(1).expect("channel will be there waiting for the pool");
        });
    }

    assert_eq!(rx.iter().take(count).fold(0, |a, b| a + b), count);
}


fn do_something(num: u32) {
    println!("{} {:?}", num, std::thread::current().id());
}