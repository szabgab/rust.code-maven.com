mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

fn main() {
    println!("TARGET                {:?}", built_info::TARGET);
    println!("PKG_VERSION           {:?}", built_info::PKG_VERSION);
    println!("RUSTC_VERSION         {:?}", built_info::RUSTC_VERSION);

    println!("GIT_VERSION           {:?}", built_info::GIT_VERSION);
    println!("GIT_DIRTY             {:?}", built_info::GIT_DIRTY);
    println!("GIT_COMMIT_HASH       {:?}", built_info::GIT_COMMIT_HASH);
    println!(
        "GIT_COMMIT_HASH_SHORT {:?}",
        built_info::GIT_COMMIT_HASH_SHORT
    );

    println!("BUILT_TIME_UTC        {:?}", built_info::BUILT_TIME_UTC);

    println!("CI_PLATFORM           {:?}", built_info::CI_PLATFORM);
}
