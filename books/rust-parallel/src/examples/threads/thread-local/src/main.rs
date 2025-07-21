use std::cell::RefCell;

thread_local! {
    pub static TEXT: RefCell<String> = const { RefCell::new(String::new()) };
}

fn main() {
    let threads = 3;

    std::thread::scope(|scope| {
        for i in 1..=threads {
            scope.spawn(move || {
                let msg = format!("doubled: {}", i * 2);
                TEXT.with_borrow_mut(|val| val.push_str(&msg));
                do_something(i)
            });
        }
    });
}

fn do_something(index: i32) {
    TEXT.with_borrow(|val| {
        println!("{index} '{val}' {:?}", std::thread::current().id());
    });
}
