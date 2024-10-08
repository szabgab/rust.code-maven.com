use sqlite::{Connection, State, Value};

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
    let mut statement = conn
        .prepare("INSERT INTO users (name) VALUES (:name);")
        .unwrap();
    statement
        .bind((":name", Value::String(name.into())))
        .unwrap();
    assert_eq!(statement.next().unwrap(), State::Done);

    let rowid = unsafe { sqlite::ffi::sqlite3_last_insert_rowid(conn.as_raw()) };
    println!("{rowid}");
}

fn list_all(conn: &Connection) {
    let mut statement = conn.prepare("SELECT * FROM users").unwrap();
    while let Ok(State::Row) = statement.next() {
        let id = statement.read::<i64, _>("id").unwrap();
        let name = statement.read::<String, _>("name").unwrap();
        println!("id={id} name = {name}");
    }
    println!("-----");
}
