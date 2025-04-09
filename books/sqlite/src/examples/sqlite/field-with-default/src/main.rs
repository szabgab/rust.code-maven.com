use sqlite::{Connection, State, Value};

fn main() {
    let connection = sqlite::open(":memory:").unwrap();
    let sql = r#"
        CREATE TABLE qa (
            question TEXT,
            answer TEXT DEFAULT "42"
        );
        "#;

    connection.execute(sql).unwrap();

    insert_qa(&connection, "Language?", "Rust");
    list_all(&connection);
    insert_qa(&connection, "Database?", "SQLite");
    list_all(&connection);
    insert_q(&connection, "Meaning of life?");
    list_all(&connection);
}

fn insert_qa(conn: &Connection, question: &str, answer: &str) {
    let mut statement = conn
        .prepare("INSERT INTO qa (question, answer) VALUES (:question, :answer);")
        .unwrap();
    statement
        .bind((":question", Value::String(question.into())))
        .unwrap();
    statement
        .bind((":answer", Value::String(answer.into())))
        .unwrap();
    assert_eq!(statement.next().unwrap(), State::Done);
}

fn insert_q(conn: &Connection, question: &str) {
    // Error { code: Some(1), message: Some("table qa has 2 columns but 1 values were supplied") }
    let mut statement = conn
        .prepare("INSERT INTO qa (question) VALUES (:question);")
        .unwrap();
    statement
        .bind((":question", Value::String(question.into())))
        .unwrap();
    assert_eq!(statement.next().unwrap(), State::Done);
}

fn list_all(conn: &Connection) {
    let mut statement = conn.prepare("SELECT * FROM qa").unwrap();
    while let Ok(State::Row) = statement.next() {
        let question = statement.read::<String, _>("question").unwrap();
        let answer = statement.read::<String, _>("answer").unwrap();
        println!("{question} - {answer}");
    }
    println!("-----");
}
