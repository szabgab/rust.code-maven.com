use serde::{Deserialize, Serialize};
use surrealdb::engine::local::{Db, Mem};
use surrealdb::sql::{Id, Thing};
use surrealdb::Surreal;

#[derive(Debug, Deserialize, Serialize)]
struct First {
    id: Thing,
    name: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Second {
    id: Thing,
    name: String,
    rgb: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Third {
    id: Thing,
    name: String,
    rgb: String,
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let dbh = Surreal::new::<Mem>(()).await?;

    dbh.use_ns("demo").use_db("demo").await?;

    add_color(&dbh, "Red").await?;
    add_color(&dbh, "Green").await?;

    let entries = get_first(&dbh).await?;
    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0].name, "Red");
    assert_eq!(entries[1].name, "Green");
    list_first(entries);

    let entries = get_second(&dbh).await?;
    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0].name, "Red");
    assert_eq!(entries[1].name, "Green");
    assert_eq!(entries[0].rgb, None);
    list_second(entries);

    add_color_with_rgb(&dbh, "Blue", "0000FF").await?;

    let entries = get_second(&dbh).await?;
    assert_eq!(entries.len(), 3);
    assert_eq!(entries[0].name, "Red");
    assert_eq!(entries[1].name, "Green");
    assert_eq!(entries[2].name, "Blue");
    assert_eq!(entries[0].rgb, None);
    assert_eq!(entries[1].rgb, None);
    assert_eq!(entries[2].rgb, Some(String::from("0000FF")));
    list_second(entries);

    // We can't do this because the table doesn't have the rgb column
    // let entries = get_third(&dbh).await?;
    // Error: Db(Serialization("failed to deserialize; expected a string, found None"))

    add_missing_rgb(&dbh, "Red", "FF0000").await?;
    add_missing_rgb(&dbh, "Green", "00FF00").await?;

    let entries = get_third(&dbh).await?;
    assert_eq!(entries.len(), 3);
    assert_eq!(entries[0].name, "Red");
    assert_eq!(entries[1].name, "Green");
    assert_eq!(entries[2].name, "Blue");
    assert_eq!(entries[0].rgb, "FF0000");
    assert_eq!(entries[1].rgb, "00FF00");
    assert_eq!(entries[2].rgb, "0000FF");
    list_third(entries);

    Ok(())
}

async fn add_color(db: &Surreal<Db>, name: &str) -> surrealdb::Result<()> {
    let _response: Option<First> = db
        .create("colors")
        .content(First {
            id: Thing::from(("colors", Id::ulid())),
            name: name.to_owned(),
        })
        .await?;

    Ok(())
}

async fn get_first(db: &Surreal<Db>) -> surrealdb::Result<Vec<First>> {
    let mut entries = db.query("SELECT id, name FROM colors").await?;
    let entries: Vec<First> = entries.take(0)?;

    Ok(entries)
}

fn list_first(entries: Vec<First>) {
    for entry in entries {
        println!("{} {}", entry.id, entry.name);
    }
    println!("-------------");
}

async fn add_color_with_rgb(db: &Surreal<Db>, name: &str, rgb: &str) -> surrealdb::Result<()> {
    let _response: Option<Second> = db
        .create("colors")
        .content(Second {
            id: Thing::from(("colors", Id::ulid())),
            name: name.to_owned(),
            rgb: Some(rgb.to_owned()),
        })
        .await?;

    Ok(())
}

async fn get_second(db: &Surreal<Db>) -> surrealdb::Result<Vec<Second>> {
    let mut entries = db.query("SELECT id, name, rgb FROM colors").await?;
    entries.take(0)
}

fn list_second(entries: Vec<Second>) {
    for entry in entries {
        println!(
            "{} {:6} {}",
            entry.id,
            entry.name,
            entry.rgb.unwrap_or("no color".to_owned())
        );
    }
    println!("-------------");
}

async fn get_third(db: &Surreal<Db>) -> surrealdb::Result<Vec<Third>> {
    let mut entries = db.query("SELECT id, name, rgb FROM colors").await?;
    let entries: Vec<Third> = entries.take(0)?;
    Ok(entries)
}

fn list_third(entries: Vec<Third>) {
    for entry in entries {
        println!("{} {:6} {}", entry.id, entry.name, entry.rgb);
    }
    println!("-------------");
}

async fn add_missing_rgb(dbh: &Surreal<Db>, name: &str, rgb: &str) -> surrealdb::Result<()> {
    let _response = dbh
        .query("UPDATE colors SET rgb=$rgb WHERE name=$name")
        .bind(("name", name.to_owned()))
        .bind(("rgb", rgb.to_owned()))
        .await?;

    Ok(())
}
