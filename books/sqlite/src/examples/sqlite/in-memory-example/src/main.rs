use sqlite::{Connection, State};

fn main() {
    let connection = sqlite::open(":memory:").unwrap();
    connection
        .execute("CREATE TABLE users (name TEXT, age INTEGER);")
        .unwrap();
    connection
        .execute("INSERT INTO users VALUES ('Alice', 42);")
        .unwrap();
    connection
        .execute("INSERT INTO users VALUES ('Bob', 23);")
        .unwrap();

    fetch(&connection, "SELECT * FROM users");
    println!("---------------------");
    fetch(&connection, "SELECT * FROM users WHERE age > 30");
}

fn fetch(connection: &Connection, query: &str) {
    let mut statement = connection.prepare(query).unwrap();

    while let Ok(State::Row) = statement.next() {
        let name: String = statement.read("name").unwrap();
        let age: i64 = statement.read("age").unwrap();
        //let name = statement.read::<String, _>("name").unwrap();
        //let age = statement.read::<u32, _>("age").unwrap();
        println!("name = {name:6} age = {age}");
    }
}
