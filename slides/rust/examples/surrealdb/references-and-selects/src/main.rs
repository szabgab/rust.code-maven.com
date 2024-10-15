use serde::{Deserialize, Serialize};
use surrealdb::engine::local::{Db, Mem};
use surrealdb::sql::{Id, Thing};
use surrealdb::Surreal;

const COURSE: &str = "course";
const STUDENT: &str = "student";

#[derive(Debug, Serialize, Deserialize)]
struct Course {
    id: Thing,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Student {
    id: Thing,
    name: String,
    courses: Vec<Thing>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct StudentWithCourses {
    id: Thing,
    name: String,
    courses: Vec<Course>,
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let db = Surreal::new::<Mem>(()).await?;

    db.use_ns("namespace").use_db("database").await?;

    add_courses(&db).await?;
    let courses = get_courses(&db).await?;

    add_students(&db, courses).await?;

    show_students_in_classes(&db).await?;

    export(&db).await?;

    Ok(())
}

async fn add_courses(db: &Surreal<Db>) -> surrealdb::Result<()> {
    for name in ["Chemistry", "Biology", "Physics"] {
        let courses: Vec<Course> = db
        .create(COURSE)
        .content(Course {
            id: Thing::from((COURSE, Id::rand())),
            name: name.to_owned(),
        })
        .await?;
        println!("course added: {courses:?}");

    }

    Ok(())
}

async fn get_courses(db: &Surreal<Db>) -> surrealdb::Result<Vec<Course>> {
    let sql = format!("SELECT * FROM $table");
    let mut results = db.query(sql).bind(("table", COURSE)).await?;
    let courses: Vec<Course> = results.take(0)?;

    for class in &courses {
        println!("get_courses: {class:?}");
    }

    Ok(courses)
}

async fn add_students(db: &Surreal<Db>, classes: Vec<Course>) -> surrealdb::Result<()> {
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
