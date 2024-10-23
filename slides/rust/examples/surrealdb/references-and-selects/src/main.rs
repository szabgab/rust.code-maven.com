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
    //println!("courses: {courses:#?}",);
    assert_eq!(courses.len(), 3);
    assert_eq!(courses[0].name, "Biology");
    assert_eq!(courses[1].name, "Chemistry");
    assert_eq!(courses[2].name, "Physics");

    add_students(&db, courses).await?;

    show_students_in_classes(&db).await?;

    Ok(())
}

async fn add_courses(db: &Surreal<Db>) -> surrealdb::Result<()> {
    for name in ["Chemistry", "Biology", "Physics"] {
        let course: Option<Course> = db
            .create(COURSE)
            .content(Course {
                id: Thing::from((COURSE, Id::rand())),
                name: name.to_owned(),
            })
            .await?;
        println!("course added: {course:?}");
    }

    Ok(())
}

async fn get_courses(db: &Surreal<Db>) -> surrealdb::Result<Vec<Course>> {
    let sql = "SELECT * FROM type::table($table) ORDER BY name";
    let mut results = db.query(sql).bind(("table", COURSE)).await?;
    let courses: Vec<Course> = results.take(0)?;

    for class in &courses {
        println!("get_courses: {class:?}");
    }

    Ok(courses)
}

async fn add_students(db: &Surreal<Db>, courses: Vec<Course>) -> surrealdb::Result<()> {
    let student: Option<Student> = db
        .create(STUDENT)
        .content(Student {
            id: Thing::from((STUDENT, Id::rand())),
            name: "Jane Doe".to_owned(),
            courses: courses.into_iter().map(|class| class.id).collect(),
        })
        .await?;
    println!("student added: {student:#?}");

    Ok(())
}

async fn show_students_in_classes(db: &Surreal<Db>) -> surrealdb::Result<()> {
    let sql = "SELECT * FROM type::table($table) FETCH courses";
    let mut results = db.query(sql).bind(("table", STUDENT)).await?;
    let students: Vec<StudentWithCourses> = results.take(0)?;
    println!("Students: {students:#?}");

    Ok(())
}
