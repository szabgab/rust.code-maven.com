fn main() {
    say_hi("before");

    fn say_hi(text: &str) {
        println!("hi {text}");
    }

    say_hi("after");

    other();
}

fn other() {
    println!("in other");

    // cannot find function `say_hi` in this scope
    //say_hi("other");
}



