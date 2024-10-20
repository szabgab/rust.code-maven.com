use serde::Deserialize;
use surrealdb::engine::local::{Db, Mem};
use surrealdb::Surreal;

#[derive(Debug, Deserialize)]
struct Person {
    name: String,
    phone: String,
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let db = Surreal::new::<Mem>(()).await?;

    db.use_ns("namespace").use_db("database").await?;

    let _response = db
        .query("DEFINE INDEX person_email ON TABLE person COLUMNS name UNIQUE")
        .await?;

    let data = vec![("Joe", "123"), ("Jane", "456"), ("Jil", "789")];

    create(&db, data).await?;
    let all = get_all(&db).await?;
    assert_eq!(all[0].name, "Jane");
    assert_eq!(all[0].phone, "456");
    assert_eq!(all[1].name, "Jil");
    assert_eq!(all[1].phone, "789");
    assert_eq!(all[2].name, "Joe");
    assert_eq!(all[2].phone, "123");
    list_all(all);

    update(&db).await?;
    let all = get_all(&db).await?;
    list_all(all);

    delete(&db).await?;
    let all = get_all(&db).await?;
    list_all(all);

    create(&db, vec![("Joe", "7777777")]).await?;
    println!("this will not happen as the previous statement panics");
    let all = get_all(&db).await?;
    list_all(all);

    Ok(())
}

async fn create(db: &Surreal<Db>, data: Vec<(&str, &str)>) -> surrealdb::Result<()> {
    for (name, phone) in data {
        let response = db
            .query("CREATE person SET  name=$name, phone=$phone")
            .bind(("name", name.to_owned()))
            .bind(("phone", phone.to_owned()))
            .await?;

        match response.check() {
            Ok(_) => {}
            Err(err) => {
                eprintln!("Could not add person: '{}'", err);
                return Err(err);
            }
        };
    }
    Ok(())
}

async fn get_all(db: &Surreal<Db>) -> surrealdb::Result<Vec<Person>> {
    let mut entries = db
        .query("SELECT name, phone FROM type::table($table) ORDER BY name ASC")
        .bind(("table", "person"))
        .await?;
    let entries: Vec<Person> = entries.take(0)?;
    Ok(entries)
}

fn list_all(entries: Vec<Person>) {
    for entry in entries {
        println!("{}: {}", entry.name, entry.phone);
    }
    println!("-------------");
}

async fn update(db: &Surreal<Db>) -> surrealdb::Result<()> {
    let name = "Jane";
    let phone = "55555555";

    let response = db
        .query("UPDATE entry SET phone=$phone WHERE name=$name")
        .bind(("name", name))
        .bind(("phone", phone))
        .await?;

    match response.check() {
        Ok(_) => Ok(()),
        Err(err) => {
            eprintln!("Could not update entry {}", err);
            Err(err)
        }
    }
}

async fn delete(db: &Surreal<Db>) -> surrealdb::Result<()> {
    let name = "Jane";

    let response = db
        .query("DELETE entry WHERE name=$name")
        .bind(("name", name))
        .await?;

    match response.check() {
        Ok(_) => Ok(()),
        Err(err) => {
            eprintln!("Could not delete entry {}", err);
            Err(err)
        }
    }
}
