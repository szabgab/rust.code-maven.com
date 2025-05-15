#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Fruit {
    fruit: String,
    color: String,
    peal: String,
}

fn main() {
    let csv_text = "
fruit,color,peal
apple,red,no
banana,yellow,yes
mango,green-yellow-red,yes

bad,format
";
    // Read CSV data as StringRecord - disregard errors
    let mut rdr = csv::Reader::from_reader(csv_text.as_bytes());
    for record in rdr.records().flatten() {
        println!("{:?}", record);
    }
    println!();

    // Read CSV data as StringRecord - report errors
    let mut rdr = csv::Reader::from_reader(csv_text.as_bytes());
    for result in rdr.records() {
        if let Ok(record) = result {
            println!("{:?}", record);
        } else {
            println!("Error parsing csv");
        }
    }
    println!();

    // Read CSV data as StringRecord - report errors with details
    let mut rdr = csv::Reader::from_reader(csv_text.as_bytes());
    for result in rdr.records() {
        match result {
            Ok(record) => println!("{:?}", record),
            Err(err) => println!("Error parsing csv {err}"),
        }
    }
    println!();

    // Read CSV data as Struct - disregard errors
    let mut rdr = csv::Reader::from_reader(csv_text.as_bytes());
    for fruit in rdr.deserialize::<Fruit>().flatten() {
        println!("{fruit:?}");
    }
}
