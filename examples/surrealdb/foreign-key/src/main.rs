use serde::{Deserialize, Serialize};
use surrealdb::engine::local::RocksDb;
//use surrealdb::opt::Resource;
use surrealdb::Surreal;

#[derive(Debug, Deserialize, Serialize)]
struct Country {
    name: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct City {
    name: String,
    country: Country,
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let database_folder = "db";

    // Start from a clean database
    if std::path::PathBuf::from(database_folder).exists() {
        std::fs::remove_dir_all(database_folder).unwrap();
    }

    let db = Surreal::new::<RocksDb>(database_folder).await?;
    db.use_ns("test").use_db("test").await?;

    for name in [String::from("Israel"), String::from("Hungary")] {
        //let country = Country { name: name};
        //db.create(Resource::from("country")).content(country).await?;

        db
            .query("CREATE country SET name=$name")
            .bind(("name", name))
            .await?;
    }

    for name in [String::from("Tel Aviv"), String::from("Haifa"), String::from("Jerusalem")] {
        // let city = City { name: name, country: Country { name : String::from("Israel")}};
        // db.create(Resource::from("city")).content(city).await?;

        db
            .query("CREATE city SET name=$name")
            .bind(("name", name))
            .bind(("country", "Israel"))
            .await?;        
    }

    // for name in [String::from("Budapest"), String::from("Székesfehérvár"), String::from("Berettyóújfalu")] {
    //     let city = City { name: name, country: Country { name : String::from("Hungary")}};
    //     db.create(Resource::from("city")).content(city).await?;
    // }

    // // I was expecting this to fail or to create "Spain" in the country table
    // for name in [String::from("Madrid"), String::from("Barcelona")] {
    //     let city = City { name: name, country: Country { name : String::from("Spain")}};
    //     db.create(Resource::from("city")).content(city).await?;
    // }


    println!("------ Countries -------");
    let mut response = db.query("SELECT * from country").await?;
    let countries: Vec<Country> = response.take(0)?;
    for country in &countries {
        println!("country: {}", country.name);
    }

    println!("------ Cities -------");
    let mut response = db.query("SELECT * from city").await?;
    let cities: Vec<City> = response.take(0)?;
    for city in &cities {
        println!("city: {}", city.name);
        //println!("city: {} in {}", city.name, city.country.name);
    }


    Ok(())
}