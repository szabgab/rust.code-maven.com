use shadow_rs::shadow;

shadow!(build);

fn main() {
    println!("branch:  {}", build::BRANCH);
    println!("version: {}", build::PKG_VERSION);
    println!("sha1:    {}", build::SHORT_COMMIT);


    let rust_version_parts = build::RUST_VERSION.split_whitespace().collect::<Vec<&str>>();
    let rust_version = if rust_version_parts.len() == 4 { rust_version_parts[1] } else { "NA" };
    println!("rust:    {}", rust_version);
}
