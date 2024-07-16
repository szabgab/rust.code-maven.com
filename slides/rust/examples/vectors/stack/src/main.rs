fn main() {
    let mut stack = vec![];
    show(&stack);

    stack.push(String::from("dog"));
    show(&stack);

    stack.push(String::from("snake"));
    show(&stack);

    stack.push(String::from("cat"));
    show(&stack);

    stack.push(String::from("turle"));
    show(&stack);

    stack.push(String::from("camel"));
    show(&stack);

    stack.pop();
    show(&stack);

    stack.pop();
    show(&stack);

    stack.shrink_to_fit();
    show(&stack);
}

fn show(stack: &Vec<String>) {
    println!("{}, {}, {:?}", stack.len(), stack.capacity(), stack);
}
