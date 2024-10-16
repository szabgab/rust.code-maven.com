use sqlite::Connection;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} [list|NAME]", args[0]);
        std::process::exit(1);
    }

    let filename = "counter.db";

    let exists = std::path::Path::new(filename).exists();

    let connection = sqlite::open(filename).unwrap();
    if !exists {
        let query = "CREATE TABLE counter (name TEXT, cnt INTEGER);";
        connection.execute(query).unwrap();
    }

    let name = &args[1];
    if name == "list" {
        list_counter(&connection);
    } else {
        count(&connection, name);
    }
}

fn count(connection: &Connection, name: &str) {
    let (cnt, new) = fetch(connection, name);

    if new {
        insert(connection, name, cnt);
    } else {
        update(connection, name, cnt);
    }
}

fn update(connection: &Connection, name: &str, cnt: i64) {
    let mut update_statement = connection
        .prepare("UPDATE counter SET cnt = :cnt WHERE name = :name;")
        .unwrap();

    match update_statement.bind_iter::<_, (_, sqlite::Value)>([
        (":cnt", sqlite::Value::Integer(cnt)),
        (":name", sqlite::Value::String(name.into())),
    ]) {
        Ok(_val) => println!("{:?}", cnt),
        Err(err) => println!("{}", err),
    }
    let _ = update_statement.next();
}

fn insert(connection: &Connection, name: &str, cnt: i64) {
    let mut insert_statement = connection
        .prepare("INSERT INTO counter VALUES (:name, :cnt);")
        .unwrap();

    match insert_statement.bind_iter::<_, (_, sqlite::Value)>([
        (":name", sqlite::Value::String(name.into())),
        (":cnt", sqlite::Value::Integer(cnt)),
    ]) {
        Ok(_val) => println!("{:?}", cnt),
        Err(err) => println!("{}", err),
    }
    let _ = insert_statement.next();
}

fn fetch(connection: &Connection, name: &str) -> (i64, bool) {
    let query = "SELECT * FROM counter WHERE name = :name";
    let mut statement = connection.prepare(query).unwrap();
    statement
        .bind::<&[(_, sqlite::Value)]>(&[(":name", sqlite::Value::String(name.into()))])
        .unwrap();

    if let Ok(sqlite::State::Row) = statement.next() {
        //println!("{} = {}", name, statement.read::<String, _>("cnt").unwrap());
        let cnt: i64 = statement.read::<String, _>("cnt").unwrap().parse().unwrap();
        (cnt + 1, false)
    } else {
        (1, true)
    }
}

fn list_counter(connection: &Connection) {
    let query = "SELECT * FROM counter";
    let mut statement = connection.prepare(query).unwrap();

    while let Ok(sqlite::State::Row) = statement.next() {
        println!(
            "{} = {}",
            statement.read::<String, _>("name").unwrap(),
            statement.read::<String, _>("cnt").unwrap()
        );
    }
}
