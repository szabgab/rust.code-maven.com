#[allow(clippy::needless_late_init)]

fn main() {
    let answer;

    // ...

    // What if we try to print the answer here already? Isn't it undefined, or null?
    // println!("The answer is {answer}");

    answer = 42;
    println!("The answer is {answer}");
}
