use sqlite::{Connection, Value, State};

fn main() {
    let connection = sqlite::open(":memory:").unwrap();
    let sql = r#"
        CREATE TABLE users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT
        );
        "#;

    connection.execute(sql).unwrap();

    insert(&connection, "Joe");
    list_all(&connection);
    insert(&connection, "Jane");
    list_all(&connection);

}

fn insert(conn: &Connection, name: &str) {
    // Error { code: Some(1), message: Some("table users has 2 columns but 1 values were supplied") }
    let mut statement = conn.prepare("INSERT INTO users VALUES (:name);").unwrap();
    //let mut statement = conn.prepare("INSERT INTO users VALUES (:id, :name);").unwrap();
    statement.bind((":name", Value::String(name.into()))).unwrap();
    //statement.bind((":id", Value::Null)).unwrap();
    assert_eq!(statement.next().unwrap(), State::Done);
}

fn list_all(conn: &Connection) {
    let mut statement = conn.prepare("SELECT * FROM users").unwrap();
    while let Ok(State::Row) = statement.next() {
        let id = statement.read::<i64, _>("name").unwrap();
        let name = statement.read::<String, _>("name").unwrap();
        println!("id={id} name = {name}");
    }
    println!("-----");
}
