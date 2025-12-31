pub fn add(left: u64, right: u64) -> u64 {
    println!("Left: {left}");
    eprintln!("Right: {right}");
    left + right
}

#[cfg(test)]
mod tests;
