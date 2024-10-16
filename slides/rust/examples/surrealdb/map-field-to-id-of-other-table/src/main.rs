use serde::{Deserialize, Serialize};
use surrealdb::engine::local::{Db, Mem};
use surrealdb::sql::{Id, Thing};
use surrealdb::Surreal;

const DANCE: &str = "dance";
const STUDENT: &str = "student";

#[derive(Debug, Serialize, Deserialize)]
struct DanceClass {
    id: Thing,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Student {
    id: Thing,
    name: String,
    classes: Vec<Thing>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct StudentClasses {
    id: Thing,
    name: String,
    classes: Vec<DanceClass>,
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let db = Surreal::new::<Mem>(()).await?;

    db.use_ns("namespace").use_db("database").await?;

    add_classes(&db).await?;
    let classes = get_classes(&db).await?;

    // the next lines are valid but then will lead to an error when fetching the data
    // classes.push(DanceClass {
    //     id: Thing::from((DANCE, Id::rand())),
    //     name: String::from("Belly dance"),
    // });

    add_students(&db, classes).await?;

    show_students_in_classes(&db).await?;

    export(&db).await?;

    Ok(())
}

async fn add_classes(db: &Surreal<Db>) -> surrealdb::Result<()> {
    for name in ["Introduction to Dancing", "Flamenco"] {
        let classes: Vec<DanceClass> = db
            .create(DANCE)
            .content(DanceClass {
                id: Thing::from((DANCE, Id::rand())),
                name: name.to_owned(),
            })
            .await?;
        println!("class added: {classes:?}");
    }

    Ok(())
}

async fn get_classes(db: &Surreal<Db>) -> surrealdb::Result<Vec<DanceClass>> {
    let sql = format!("SELECT * FROM dance");
    let mut results = db.query(sql).await?;
    let classes: Vec<DanceClass> = results.take(0)?;

    for class in &classes {
        println!("get_classes: {class:?}");
    }

    Ok(classes)
}

async fn add_students(db: &Surreal<Db>, classes: Vec<DanceClass>) -> surrealdb::Result<()> {
    let students: Vec<Student> = db
        .create(STUDENT)
        .content(Student {
            id: Thing::from((STUDENT, Id::rand())),
            name: "Jane Doe".to_owned(),
            classes: classes.into_iter().map(|class| class.id).collect(),
        })
        .await?;
    println!("student added: {students:#?}");

    Ok(())
}

async fn show_students_in_classes(db: &Surreal<Db>) -> surrealdb::Result<()> {
    let sql = format!("SELECT * FROM {STUDENT} FETCH classes");
    let mut results = db.query(sql).await?;
    let students: Vec<StudentClasses> = results.take(0)?;
    println!("Students: {students:#?}");

    Ok(())
}

async fn export(db: &Surreal<Db>) -> surrealdb::Result<()> {
    db.export("out.sql").await?;
    Ok(())
}
