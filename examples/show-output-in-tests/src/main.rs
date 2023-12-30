fn main() {
    answer();
}

fn answer() -> u32 {
    println!("STDOUT in code");
    eprintln!("STDERR in code");
    42
}

#[test]
fn test_function() {
    println!("STDOUT In test_function");
    eprintln!("STDERR In test_function");

    assert_eq!(answer(), 42);
}
