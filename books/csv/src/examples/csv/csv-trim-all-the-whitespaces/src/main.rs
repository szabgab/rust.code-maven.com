#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Planet {
    #[serde(alias = "Planet name")]
    name: String,

    #[serde(alias = "Distance (AU)")]
    distance: String,

    #[serde(alias = "Mass")]
    mass: String,
}

fn main() {
    let filepath = "planets.csv";

    read_and_print_file(filepath);
    println!();

    for result in read_file_return_results(filepath) {
        match result {
            Ok(planet) => println!("{planet:?}"),
            Err(err) => println!("Error parsing csv {err}"),
        }
    }
    println!();

    for planet in read_file_return_planets(filepath) {
        println!("{planet:?}");
    }
}

fn read_and_print_file(filepath: &str) {
    let csv_text = std::fs::read_to_string(filepath).expect("Error reading file");
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .trim(csv::Trim::All)
        .from_reader(csv_text.as_bytes());

    for result in rdr.deserialize::<Planet>() {
        match result {
            Ok(record) => println!("{:?}", record),
            Err(err) => println!("Error parsing csv {err}"),
        }
    }
}

fn read_file_return_results(filepath: &str) -> Vec<Result<Planet, csv::Error>> {
    let csv_text = std::fs::read_to_string(filepath).expect("Error reading file");
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .trim(csv::Trim::All)
        .from_reader(csv_text.as_bytes());

    rdr.deserialize::<Planet>().collect::<Vec<_>>()
}

fn read_file_return_planets(filepath: &str) -> Vec<Planet> {
    let csv_text = std::fs::read_to_string(filepath).expect("Error reading file");
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .trim(csv::Trim::All)
        .from_reader(csv_text.as_bytes());

    rdr.deserialize::<Planet>()
        .filter_map(|entry| entry.ok())
        .collect::<Vec<_>>()
}
