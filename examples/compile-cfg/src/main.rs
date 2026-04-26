fn main() {
    #[cfg(windows)]
    println!("Hello, Windows!");
    #[cfg(unix)]
    println!("Hello, Unix!");
    #[cfg(target_os = "macos")]
    println!("Hello, macOS!");
    #[cfg(windows)]
    println!("Hello, Windows!");
    println!("After all");
}
 
