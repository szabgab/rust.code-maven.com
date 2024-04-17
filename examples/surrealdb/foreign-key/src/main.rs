use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::Ws;
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
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;

    db.use_ns("test").use_db("test").await?;

    let places: HashMap<&str, Vec<&str>> = HashMap::from([
        ("Israel", vec!["Tel Aviv", "Haifa", "Jerusalem"]),
        ("Hungary", vec!["Debrecen", "Budapest"]),
    ]);
    //dbg!(places);

    db.query("DELETE country").await?;

    for name in places.keys() {
        println!("{name}");
        //let country = Country { name: name};
        //db.create(Resource::from("country")).content(country).await?;

        db.query("CREATE country SET name=$name")
            .bind(("name", name))
            .await?;
    }

    // for name in [
    //     String::from("Mildendo"),
    //     String::from("Plips"),
    //     String::from("Wiggywack"),
    // ] {
    //     // let city = City { name: name, country: Country { name : String::from("Lilliput")}};
    //     // db.create(Resource::from("city")).content(city).await?;

    //     db
    //         .query("CREATE city SET name=$name, country=(SELECT VALUE id FROM country WHERE name=$country LIMIT 1)[0]")
    //         .bind(("name", &name))
    //         .bind(("country", "Lilliput"))
    //         .await?;
    // }

    // for name in [String::from("Blefuscu-city")] {
    //     //let city = City { name: name, country: Country { name : String::from("Blefuscu")}};
    //     //db.create(Resource::from("city")).content(city).await?;

    //     db
    //     .query("CREATE city SET name=$name, country=(SELECT VALUE id FROM country WHERE name=$country LIMIT 1)[0]")
    //         .bind(("name", name))
    //         .bind(("country", "Blefuscu"))
    //         .await?;
    // }

    // // I was expecting this to fail or to create "Spain" in the country table
    // for name in [String::from("Madrid"), String::from("Barcelona")] {
    //     let city = City { name: name, country: Country { name : String::from("Spain")}};
    //     db.create(Resource::from("city")).content(city).await?;
    // }

    // for name in [String::from("Madrid")] {
    //     //let city = City { name: name, country: Country { name : String::from("Blefuscu")}};
    //     //db.create(Resource::from("city")).content(city).await?;

    //     db
    //     .query("CREATE city SET name=$name, country=(SELECT VALUE id FROM country WHERE name=$country LIMIT 1)[0]")
    //         .bind(("name", name))
    //         .bind(("country", "Spain"))
    //         .await?;
    // }

    // let mut backup = db.export(()).await?;
    // while let Some(result) = backup.next().await {
    //     match result {
    //         Ok(bytes) => {
    //             // Do something with the bytes received...
    //         }
    //         Err(error) => {
    //             // Handle the export error
    //         }
    //     }
    // }
    // //let info = db.info();

    // //println!("{}", info);

    println!("------ Countries -------");
    let mut response = db.query("SELECT * FROM country").await?;
    let countries: Vec<Country> = response.take(0)?;
    for country in &countries {
        println!("country: {}", country.name);
    }

    // println!("------ Cities -------");
    // let mut response = db.query("SELECT * FROM city FETCH country").await?;
    // let cities: Vec<City> = response.take(0)?;
    // for city in &cities {
    //     //println!("city: {}", city.name);
    //     println!("city: {:15} in {}", city.name, city.country.name);
    // }


    
    
    Ok(())
}
