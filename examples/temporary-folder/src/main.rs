use tempdir::TempDir;
fn main() {
    temp_dir();
}

fn temp_dir() {
    let tmp_dir = TempDir::new("example").unwrap();
    println!("tempdir: {:?}", tmp_dir);
}
