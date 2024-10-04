use serde::{Deserialize, Serialize};
use surrealdb::engine::local::{Db, Mem};
use surrealdb::sql::Thing;
use surrealdb::Surreal;

#[derive(Debug, Serialize, Deserialize)]
struct RSVP {
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
        .query("DEFINE INDEX rsvp_id ON TABLE rsvp COLUMNS uid UNIQUE")
        .await?;

    let _response = dbh.query("DELETE rsvp").await?.check();
    list(&dbh).await?;

    set(&dbh, 1, true).await?;
    set(&dbh, 2, true).await?;
    set(&dbh, 3, false).await?;

    list(&dbh).await?;

    set(&dbh, 1, false).await?;
    set(&dbh, 3, true).await?;

    list(&dbh).await?;

    Ok(())
}

async fn set(dbh: &Surreal<Db>, uid: u32, status: bool) -> surrealdb::Result<()> {
    //println!("set {uid} to status: {status}");

    let mut response = dbh
        .query("SELECT * FROM rsvp where uid=$uid")
        .bind(("uid", uid))
        .await?;
    let rows: Vec<RSVP> = response.take(0)?;
    if let Some(_rsvp) = rows.first() {
        //println!("Update: {rsvp:?} with {status}");
        dbh.query("UPDATE rsvp SET status=$status WHERE uid=$uid")
            .bind(("status", status))
            .bind(("uid", uid))
            .await?;
    } else {
        let _created: Option<Record> = dbh
            .create("rsvp")
            .content(RSVP {
                uid: uid,
                status: status,
            })
            .await?;
        //println!("created: {created:?}");
    }

    Ok(())
}

async fn list(db: &Surreal<Db>) -> surrealdb::Result<()> {
    let statuses: Vec<RSVP> = db.select("rsvp").await?;
    println!("List:");
    for status in statuses {
        println!("   {:?}", status);
    }

    Ok(())
}
