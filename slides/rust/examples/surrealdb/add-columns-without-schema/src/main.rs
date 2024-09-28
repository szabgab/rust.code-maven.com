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
    list_first(&dbh).await?;
    println!("-------------");

    list_second(&dbh).await?;
    println!("-------------");

    add_color_with_rgb(&dbh, "Blue", "0000FF").await?;

    list_second(&dbh).await?;
    println!("-------------");

    // We can't do this because the table doesn't have the rgb column
    // list_third(&dbh).await?;
    // println!("-------------");

    add_missing_rgb(&dbh, "Red", "FF0000").await?;
    add_missing_rgb(&dbh, "Green", "00FF00").await?;

    list_third(&dbh).await?;
    println!("-------------");

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

async fn list_first(db: &Surreal<Db>) -> surrealdb::Result<()> {
    let mut entries = db.query("SELECT id, name FROM colors").await?;
    let entries: Vec<First> = entries.take(0)?;
    for entry in entries {
        println!("{} {}", entry.id, entry.name);
    }

    Ok(())
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

async fn list_second(db: &Surreal<Db>) -> surrealdb::Result<()> {
    let mut entries = db.query("SELECT id, name, rgb FROM colors").await?;
    let entries: Vec<Second> = entries.take(0)?;
    for entry in entries {
        println!(
            "{} {:6} {}",
            entry.id,
            entry.name,
            entry.rgb.unwrap_or("no color".to_owned())
        );
    }

    Ok(())
}

async fn list_third(db: &Surreal<Db>) -> surrealdb::Result<()> {
    let mut entries = db.query("SELECT id, name, rgb FROM colors").await?;
    let entries: Vec<Third> = entries.take(0)?;
    for entry in entries {
        println!("{} {:6} {}", entry.id, entry.name, entry.rgb);
    }

    Ok(())
}

async fn add_missing_rgb(dbh: &Surreal<Db>, name: &str, rgb: &str) -> surrealdb::Result<()> {
    let _response = dbh
        .query("UPDATE colors SET rgb=$rgb WHERE name=$name")
        .bind(("name", name.to_owned()))
        .bind(("rgb", rgb.to_owned()))
        .await?;

    Ok(())
}
