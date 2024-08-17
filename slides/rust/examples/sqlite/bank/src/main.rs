use sqlite::{Connection, State};

use clap::Parser;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
enum Action {
    Setup {},
    Add {
        account: String,
        amount: i64,
    },
    Transfer {
        from: String,
        to: String,
        amount: i64,

        #[arg(long)]
        panic: bool,

        #[arg(long)]
        transaction: bool,
    },
    Show {},
}

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    action: Action,
}

fn main() {
    let args = Cli::parse();

    let filename = "bank.db";
    let connection = sqlite::open(filename).unwrap();

    match &args.action {
        Action::Setup {} => {
            setup_bank(&connection);
        }
        Action::Add { account, amount } => {
            println!("{:?} {:?}", account, amount);
            add(&connection, account, *amount);
        }
        Action::Transfer {
            from,
            to,
            amount,
            panic,
            transaction,
        } => {
            println!("{:?} {:?} {:?}", from, to, amount);
            transfer(&connection, from, to, *amount, *panic, *transaction)
        }
        Action::Show {} => {
            show(&connection);
        }
    }
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
}

fn transfer(
    connection: &Connection,
    from: &str,
    to: &str,
    amount: i64,
    fail: bool,
    transaction: bool,
) {
    let sql = r#"
        UPDATE bank SET balance = (
            SELECT balance FROM bank WHERE name = :name
        ) + :amount WHERE name = :name;
    "#;

    if transaction {
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
        let total = statement.read::<i64, _>("total").unwrap();
        println!("Total: {:>4}", total);
    }
    println!("-----");
}

fn add(connection: &Connection, name: &str, amount: i64) {
    let mut statement = connection
        .prepare("INSERT INTO bank (name, balance) VALUES (:name, :amount);")
        .unwrap();
    statement.bind((":name", name)).unwrap();
    statement.bind((":amount", amount)).unwrap();
    statement.next().unwrap();
}
