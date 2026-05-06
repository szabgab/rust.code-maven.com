use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};
use std::{error::Error, fs, str::FromStr};

#[derive(Debug, Clone)]
struct GradeRecord {
    student: String,
    math: Option<u32>,
    chemistry: Option<u32>,
    biology: Option<u32>,
    physics: Option<u32>,
    literature: Option<u32>,
    sport: Option<u32>,
    drawing: Option<u32>,
}

fn parse_score(value: &str) -> Result<Option<u32>, Box<dyn Error>> {
    let value = value.trim();
    if value == "-" || value.is_empty() {
        Ok(None)
    } else {
        Ok(Some(value.parse()?))
    }
}

fn read_grades_csv(path: &str) -> Result<Vec<GradeRecord>, Box<dyn Error>> {
    let content = fs::read_to_string(path)?;
    let mut rows = Vec::new();

    for line in content.lines().skip(1) {
        if line.trim().is_empty() {
            continue;
        }

        let cols: Vec<&str> = line.split(',').collect();
        if cols.len() != 8 {
            return Err(format!("Invalid row with {} columns: {line}", cols.len()).into());
        }

        rows.push(GradeRecord {
            student: cols[0].trim().to_string(),
            math: parse_score(cols[1])?,
            chemistry: parse_score(cols[2])?,
            biology: parse_score(cols[3])?,
            physics: parse_score(cols[4])?,
            literature: parse_score(cols[5])?,
            sport: parse_score(cols[6])?,
            drawing: parse_score(cols[7])?,
        });
    }

    Ok(rows)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let grades = read_grades_csv("grades.csv")?;

    let options = SqliteConnectOptions::from_str("sqlite://grades.db")?.create_if_missing(true);
    let pool = SqlitePool::connect_with(options).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    for grade in &grades {
        sqlx::query(
            "
            INSERT INTO grades (student, math, chemistry, biology, physics, literature, sport, drawing)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT(student) DO UPDATE SET
                math = excluded.math,
                chemistry = excluded.chemistry,
                biology = excluded.biology,
                physics = excluded.physics,
                literature = excluded.literature,
                sport = excluded.sport,
                drawing = excluded.drawing
            ",
        )
        .bind(&grade.student)
        .bind(grade.math.map(i64::from))
        .bind(grade.chemistry.map(i64::from))
        .bind(grade.biology.map(i64::from))
        .bind(grade.physics.map(i64::from))
        .bind(grade.literature.map(i64::from))
        .bind(grade.sport.map(i64::from))
        .bind(grade.drawing.map(i64::from))
        .execute(&pool)
        .await?;
    }

    println!("Loaded {} grade rows into grades.db", grades.len());
    Ok(())
}
