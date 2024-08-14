use sqlite::State;

fn main() {
    let connection = sqlite::open(":memory:").unwrap();
    connection.execute("CREATE TABLE users (name TEXT, age INTEGER, grade INTEGER);").unwrap();
    connection.execute("INSERT INTO users VALUES ('Alice', 42, 80);").unwrap();
    connection.execute("INSERT INTO users VALUES ('Bob', 79, 70);").unwrap();

    let query = "SELECT COUNT(*) cnt FROM users";
    let mut statement = connection.prepare(query).unwrap();
    if let Ok(State::Row) = statement.next() {
        println!("\ncount = {}", statement.read::<i64, _>("cnt").unwrap());
    } else {
        println!("trouble");
    }
}
