use sqlite::State;

fn main() {
    let connection = sqlite::open(":memory:").unwrap();
    connection
        .execute("CREATE TABLE users (name TEXT, age INTEGER, grade INTEGER);")
        .unwrap();
    connection
        .execute("INSERT INTO users VALUES ('Alice', 42, 80);")
        .unwrap();
    connection
        .execute("INSERT INTO users VALUES ('Bob', 79, 70);")
        .unwrap();

    let age = 50;
    let grade = 50;
    let query = "SELECT * FROM users WHERE age > :age AND grade > :grade";
    let mut statement = connection.prepare(query).unwrap();
    statement.bind((1, age)).unwrap();
    statement.bind((2, grade)).unwrap();

    while let Ok(State::Row) = statement.next() {
        println!("name = {}", statement.read::<String, _>("name").unwrap());
        println!("age = {}", statement.read::<i64, _>("age").unwrap());
        println!("grade = {}", statement.read::<i64, _>("grade").unwrap());
    }
}
