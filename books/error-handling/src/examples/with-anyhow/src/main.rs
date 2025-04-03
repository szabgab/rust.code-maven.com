use anyhow::bail;

fn main() {
    match do_something(42) {
        Ok(_) => println!("Success!"),
        Err(err) => println!("Error: {}", err),
    }

    match do_something(23) {
        Ok(_) => println!("Success!"),
        Err(err) => println!("Error: {}", err),
    }

}


fn do_something(answer: i32) -> anyhow::Result<()> {
    println!("Doing something!");
    if answer != 42 {
        bail!("An error occurred");
    }

    println!("The answer is 42!");
    Ok(())
}