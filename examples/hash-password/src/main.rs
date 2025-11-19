use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use std::io::{self, Write};

fn main() {
    // Get password from user input
    print!("Enter password to hash: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let password = input.trim().as_bytes();

    // Generate a random salt
    let salt = SaltString::generate(&mut OsRng);

    // Initialize Argon2 with default configuration
    let argon2 = Argon2::default();

    // Hash the password
    let password_hash = argon2.hash_password(password, &salt).unwrap();

    println!("\nPassword hash: {}", password_hash);

    // Demonstrate verification
    println!("\nVerifying the password...");
    let hash_string = password_hash.to_string();
    println!("\nPassword hash: {}", hash_string);
    let parsed_hash = PasswordHash::new(&hash_string).unwrap();

    match argon2.verify_password(password, &parsed_hash) {
        Ok(_) => println!("✓ Password verification successful!"),
        Err(_) => println!("✗ Password verification failed!"),
    }
}
