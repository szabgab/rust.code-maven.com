use clap::Parser;
use clap::Subcommand;
use sqlite::Connection;
use sqlite::State;

#[derive(Subcommand, Debug)]
enum Action {
    Setup {},
    Show {},
    User { name: String },
    Group { name: String, owner: String },
}

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    action: Action,
}

fn main() {
    let args = Cli::parse();

    let connection = sqlite::open("ug.db").unwrap();
    connection.execute("PRAGMA foreign_keys = ON;").unwrap();

    match &args.action {
        Action::Setup {} => {
            setup(&connection);
        }
        Action::Show {} => {
            show(&connection);
        }
        Action::User { name } => {
            add_user(&connection, name);
        }
        Action::Group { name, owner } => {
            add_group(&connection, name, owner);
        }
    }
}

fn setup(connection: &Connection) {
    let sql = r#"
        CREATE TABLE users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT UNIQUE
        );
        CREATE TABLE groups (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT UNIQUE,
            owner INTEGER,
            FOREIGN KEY(owner) REFERENCES users(id)
        )
    "#;
    connection.execute(sql).unwrap();
}

fn show(connection: &Connection) {
    let mut statement = connection.prepare("SELECT * FROM users;").unwrap();
    while let Ok(State::Row) = statement.next() {
        let id = statement.read::<i64, _>("id").unwrap();
        let name = statement.read::<String, _>("name").unwrap();
        println!("{id} {name}");
    }
    println!("---------");

    let sql = r#"
        SELECT
            users.name uname,
            users.id uid,
            groups.name gname,
            groups.id gid
        FROM users, groups WHERE groups.owner=users.id;"
    "#;
    let mut statement = connection.prepare(sql).unwrap();
    while let Ok(State::Row) = statement.next() {
        let uid = statement.read::<i64, _>("uid").unwrap();
        let uname = statement.read::<String, _>("uname").unwrap();
        let gname = statement.read::<String, _>("gname").unwrap();
        let gid = statement.read::<i64, _>("gid").unwrap();
        println!("{uname}({uid}) is the owner of {gname}({gid})");
    }
}

fn add_user(connection: &Connection, name: &str) {
    let sql = "INSERT INTO users (name) VALUES (:name);";
    let mut stmt = connection.prepare(sql).unwrap();
    stmt.bind((":name", name)).unwrap();
    stmt.next().unwrap();
}

fn add_group(connection: &Connection, name: &str, owner: &str) {
    let id = get_user_id(connection, owner);

    let sql = "INSERT INTO groups (name, owner) VALUES (:name, :owner);";
    let mut stmt = connection.prepare(sql).unwrap();
    stmt.bind((":name", name)).unwrap();
    stmt.bind((":owner", id)).unwrap();
    stmt.next().unwrap();
}

fn get_user_id(connection: &Connection, owner: &str) -> i64 {
    if let Ok(id) = owner.parse::<i64>() {
        return id;
    }

    let sql = "SELECT id FROM users WHERE name=:name;";
    let mut statement = connection.prepare(sql).unwrap();
    statement.bind((":name", owner)).unwrap();
    if let Ok(State::Row) = statement.next() {
        return statement.read::<i64, _>("id").unwrap();
    }

    panic!("Could not find user {owner}");
}
