use sqlite::{Connection, State};

fn main() {
    let data = [("Alice", 42, 80), ("Bob", 79, 70)];

    let connection = sqlite::open(":memory:").unwrap();
    connection
        .execute("CREATE TABLE users (name TEXT, age INTEGER, grade INTEGER);")
        .unwrap();

    let query = "INSERT INTO users VALUES (?, ?, ?)";
    let mut insert_statement = connection.prepare(query).unwrap();

    for row in data {
        insert_statement.bind((1, row.0)).unwrap();
        insert_statement.bind((2, row.1)).unwrap();
        insert_statement.bind((3, row.2)).unwrap();
        let _ = insert_statement.next();
        let _ = insert_statement.reset();
    }

    let age = 50;
    let grade = 50;
    let query = "SELECT * FROM users WHERE age > ? AND grade > ?";
    let mut statement = connection.prepare(query).unwrap();
    statement.bind((1, age)).unwrap();
    statement.bind((2, grade)).unwrap();

    while let Ok(State::Row) = statement.next() {
        println!("name = {}", statement.read::<String, _>("name").unwrap());
        println!("age = {}", statement.read::<i64, _>("age").unwrap());
        println!("grade = {}", statement.read::<i64, _>("grade").unwrap());
    }

    count(&connection);
}

fn count(connection: &Connection) {
    let query = "SELECT COUNT(*) cnt FROM users";
    let mut statement = connection.prepare(query).unwrap();
    if let Ok(State::Row) = statement.next() {
        println!("\ncount = {}", statement.read::<i64, _>("cnt").unwrap());
    } else {
        println!("trouble");
    }
}
