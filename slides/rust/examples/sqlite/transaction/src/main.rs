use sqlite::{Connection, State};

use clap::Parser;
use clap::ValueEnum;

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Action {
    Plain,
    Panic,
    Transaction,
    Show,
}

#[derive(Parser, Debug)]
struct Cli {
    action: Action,
}

fn main() {
    let args = Cli::parse();

    let filename = "bank.db";
    let exists = std::path::PathBuf::from(filename).exists();
    let connection = sqlite::open(filename).unwrap();

    if !exists {
        setup_bank(&connection);
    }

    match args.action {
        Action::Plain => transfer(&connection, "Mary", "Jane", 100, false, false),
        Action::Panic => transfer(&connection, "Mary", "Jane", 100, true, false),
        Action::Transaction => transfer(&connection, "Mary", "Jane", 100, true, true),
        Action::Show => (),
    };
    show(&connection);
    std::fs::remove_file(filename).unwrap();
}

fn setup_bank(connection: &Connection) {
    connection
        .execute(
            r#"
        CREATE TABLE bank (
            name TEXT PRIMARY KEY,
            balance INTEGER NOT NULL
        );
    "#,
        )
        .unwrap();
    connection
        .execute("INSERT INTO bank (name, balance) VALUES ('Jane', 0);")
        .unwrap();
    connection
        .execute("INSERT INTO bank (name, balance) VALUES ('Mary', 1000);")
        .unwrap();
    connection
        .execute("INSERT INTO bank (name, balance) VALUES ('Ann', 1000);")
        .unwrap();
}

fn transfer(
    connection: &Connection,
    from: &str,
    to: &str,
    amount: i64,
    fail: bool,
    transaction: bool,
) {
    let sql = r#"UPDATE bank SET balance = (SELECT balance FROM bank WHERE name = :name) + :amount WHERE name = :name;"#;

    if transaction {
        println!("BEGIN");
        connection.prepare("BEGIN").unwrap().next().unwrap();
    }
    let mut statement = connection.prepare(sql).unwrap();
    statement.bind((":name", from)).unwrap();
    statement.bind((":amount", -amount)).unwrap();
    statement.next().unwrap();
    if fail {
        panic!("Problem");
    }

    statement.reset().unwrap();
    statement.bind((":name", to)).unwrap();
    statement.bind((":amount", amount)).unwrap();
    statement.next().unwrap();

    if transaction {
        println!("COMMIT");
        connection.prepare("COMMIT").unwrap().next().unwrap();
    }
}

fn show(connection: &Connection) {
    let mut statement = connection.prepare("SELECT * FROM bank;").unwrap();
    while let Ok(State::Row) = statement.next() {
        let name = statement.read::<String, _>("name").unwrap();
        let balance = statement.read::<i64, _>("balance").unwrap();
        println!("{name:5}: {:>4}", balance);
    }

    let mut statement = connection
        .prepare("SELECT SUM(balance) AS total FROM bank;")
        .unwrap();
    if let Ok(State::Row) = statement.next() {
        let total = statement.read::<String, _>("total").unwrap();
        println!("Total: {:>4}", total);
    }
    //
    println!("-----");
}
