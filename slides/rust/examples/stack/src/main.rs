fn main() {
    let mut stack = vec![];
    show(&stack);

    stack.push("dog");
    show(&stack);

    stack.push("snake");
    show(&stack);

    stack.push("cat");
    show(&stack);

    stack.push("turle");
    show(&stack);

    stack.push("camel");
    show(&stack);

    stack.pop();
    show(&stack);

    stack.pop();
    show(&stack);

    stack.shrink_to_fit();
    show(&stack);
}

fn show(stack: &Vec<&str>) {
    println!("{}, {}, {:?}", stack.len(), stack.capacity(), stack);
}

/*

0, 0, []
1, 4, ["dog"]
2, 4, ["dog", "snake"]
3, 4, ["dog", "snake", "cat"]
4, 4, ["dog", "snake", "cat", "turle"]
5, 8, ["dog", "snake", "cat", "turle", "camel"]
4, 8, ["dog", "snake", "cat", "turle"]
3, 8, ["dog", "snake", "cat"]
3, 3, ["dog", "snake", "cat"]

*/