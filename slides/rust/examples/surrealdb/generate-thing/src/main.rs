
use surrealdb::sql::{Id, Thing};

fn main() {
    let id = Thing::from(("person", Id::rand()));
    println!("id {id:?}");
    println!("id {id}");
    println!("id {}", id.to_raw());
    println!();

    // Generate a new random ULID = Universally Unique Lexicographically Sortable Identifier https://github.com/ulid/spec
    let ulid = Thing::from(("person", Id::ulid()));
    println!("ulid {ulid:?}");
    println!("ulid {ulid}");
    println!("ulid {}", ulid.to_raw());
    println!();

    let uuid = Thing::from(("person", Id::uuid()));
    println!("uuid {uuid:?}");
    println!("uuid {uuid}");
    println!("uuid {}", uuid.to_raw());
}
