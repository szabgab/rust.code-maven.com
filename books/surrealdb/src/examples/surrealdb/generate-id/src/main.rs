use surrealdb::sql::Id;

fn main() {
    let id = Id::rand(); // Generate a new random ID
    println!("id {id:?}");
    println!("id {id}");
    println!("id {}", id.to_raw());
    println!();

    let ulid = Id::ulid(); // Generate a new random ULID = Universally Unique Lexicographically Sortable Identifier https://github.com/ulid/spec
    println!("ulid {ulid:?}");
    println!("ulid {ulid}");
    println!("ulid {}", ulid.to_raw());
    println!();

    let uuid = Id::uuid(); // Generate a new random UUID
    println!("uuid {uuid:?}");
    println!("uuid {uuid}");
    println!("uuid {}", uuid.to_raw());
}
