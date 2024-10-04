use serde::{Deserialize, Serialize};
use surrealdb::engine::local::{Db, Mem};
use surrealdb::sql::Thing;
use surrealdb::Surreal;

#[derive(Debug, Serialize, Deserialize)]
struct Toggle {
    uid: u32,
    status: bool,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let dbh = Surreal::new::<Mem>(()).await?;

    dbh.use_ns("demo").use_db("demo").await?;
    let _response = dbh
        .query("DEFINE INDEX toggle_id ON TABLE toggle COLUMNS uid UNIQUE")
        .await?;

    let _response = dbh.query("DELETE toggle").await?.check();
    list(&dbh).await?;

    set(&dbh, 1, true).await?;
    set(&dbh, 2, true).await?;
    set(&dbh, 3, false).await?;

    list(&dbh).await?;

    toggle_status(&dbh, 1).await?;
    toggle_status(&dbh, 3).await?;

    //toggle_status(&dbh, 4).await?;
    list(&dbh).await?;

    Ok(())
}

async fn set(dbh: &Surreal<Db>, uid: u32, status: bool) -> surrealdb::Result<()> {
    //println!("set {uid} to status: {status}");

    let _created: Option<Record> = dbh
        .create("toggle")
        .content(Toggle {
            uid: uid,
            status: status,
        })
        .await?;

    Ok(())
}

async fn toggle_status(dbh: &Surreal<Db>, uid: u32) -> surrealdb::Result<()> {
    let mut response = dbh
        .query("SELECT * FROM toggle where uid=$uid")
        .bind(("uid", uid))
        .await?;
    let rows: Vec<Toggle> = response.take(0)?;
    let Some(toggle) = rows.first() else {
        return Ok(()); // Should return Err()
    };

    dbh.query("UPDATE toggle SET status=$status WHERE uid=$uid")
        .bind(("status", !toggle.status))
        .bind(("uid", uid))
        .await?;

    Ok(())
}

async fn list(db: &Surreal<Db>) -> surrealdb::Result<()> {
    let statuses: Vec<Toggle> = db.select("toggle").await?;
    println!("List:");
    for status in statuses {
        println!("   {:?}", status);
    }

    Ok(())
}
