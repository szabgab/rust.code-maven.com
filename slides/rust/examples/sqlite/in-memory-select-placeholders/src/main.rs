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

    // bind in one step
    // 1 and 2 refer to the order number of the placeholders
    statement
        .bind::<&[(_, sqlite::Value)]>(&[(1, age.into()), (2, grade.into())])
        .unwrap();

    // We cab also use names
    // statement.bind::<&[(_, sqlite::Value)]>(&[(":age", age.into()), (":grade", grade.into())]).unwrap();
    //

    // We can also prepare the vector up-front
    // let params = vec![(1, age.into()), (2, grade.into())];
    // let params = vec![(":age", age.into()), (":grade", grade.into())];
    // statement.bind::<&[(_, sqlite::Value)]>(&params).unwrap();

    while let Ok(State::Row) = statement.next() {
        println!("name = {}", statement.read::<String, _>("name").unwrap());
        println!("age = {}", statement.read::<i64, _>("age").unwrap());
        println!("grade = {}", statement.read::<i64, _>("grade").unwrap());
    }
}
