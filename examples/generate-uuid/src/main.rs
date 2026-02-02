use uuid::Uuid;

fn main() {
    // node_id is unique to this process
    let node_id = &[1, 2, 3, 4, 5, 6];
    let id = Uuid::now_v1(node_id);
    println!("uuid v1: {}", id);

    let id = Uuid::new_v3(&Uuid::NAMESPACE_DNS, b"rust-lang.org");
    println!("uuid v3: {}", id);

    let id = Uuid::new_v4();
    println!("uuid v4: {}", id);

    let id = Uuid::new_v5(&Uuid::NAMESPACE_DNS, b"rust-lang.org");
    println!("uuid v5: {}", id);

    //    let context = Context::new(random_seed());
    //    let ts = Timestamp::from_unix(context, 1497624119, 1234);
    //    let id = Uuid::new_v6(ts, node_id);
    //    println!("uuid v6: {}", id);
    //
    use short_uuid::ShortUuid;

    let id = ShortUuid::generate();
    println!("short_uuid: {}", id);
}
