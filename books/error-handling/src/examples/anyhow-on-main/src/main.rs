use anyhow::bail;

fn main() -> anyhow::Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        bail!("Usage: {} <number>", args[0]);
    }
    let number: i32 = args[1].parse().map_err(|_| anyhow::anyhow!("Invalid number"))?;
    do_something(number)?;

    Ok(())
}


fn do_something(answer: i32) -> anyhow::Result<()> {
    println!("Doing something!");
    if answer != 42 {
        bail!("Invalid number: {}", answer);
    }

    println!("The answer is 42!");
    Ok(())
}