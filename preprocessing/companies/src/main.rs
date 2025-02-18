use serde::Deserialize;
//use thousands::Separable;

// TODO: separate pages by language
// TODO: separate pages by country
// TODO: order by date

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
struct Address {
    country: String,
    //state: String,
    #[serde(default = "get_empty_string")]
    city: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
struct Person {
    name: String,
    url: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
struct Proof {
    url: String,
    title: String,
    ptype: String,
    language: String,
    date: String,
    people: Vec<Person>,
    description: Option<String>,
    from: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
struct Company {
    name: String,
    url: String,
    addresses: Vec<Address>,
    proofs: Vec<Proof>,
}

fn main() {
    companies();
}

fn get_empty_string() -> String {
    String::new()
}

fn companies() {
    let filename = std::path::Path::new("../../examples/companies.yaml");
    let yaml_string = std::fs::read_to_string(filename).unwrap();
    let companies: Vec<Company> = match serde_yaml::from_str(&yaml_string) {
        Ok(value) => value,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };
    //println!("{:?}", data);

    //generate_md_file(&companies);

    let split = companies
        .into_iter()
        .map(|company| {
            if company.proofs.len() == 1 {
                vec![company]
            } else {
                company
                    .proofs
                    .iter()
                    .map(|proof| {
                        let mut comp = company.clone();
                        comp.proofs = vec![proof.clone()];
                        comp
                    })
                    .collect::<Vec<Company>>()
            }
        })
        .collect::<Vec<Vec<Company>>>()
        .concat();
    generate_md_file(&split);
}

fn generate_md_file(companies: &[Company]) {
    let mut text = String::from(include_str!("header.md"));
    let footer = String::from(include_str!("footer.md"));

    for company in companies.iter() {
        text.push_str(format!("## [{}]({})\n\n", company.name, company.url,).as_str());
        for address in company.addresses.iter() {
            text.push_str(format!("({}, {})\n", address.country, address.city).as_str());
        }

        text.push('\n');

        for proof in company.proofs.iter() {
            text.push_str(
                format!(
                    "* [{}]({}) (From {} in {})\n",
                    proof.title, proof.url, proof.date, proof.language
                )
                .as_str(),
            );
            match &proof.description {
                Some(description) => {
                    text.push_str(description);
                    text.push('\n');
                    text.push('\n');
                }
                None => {}
            };
            for person in &proof.people {
                text.push_str(format!("    * [{}]({})\n", person.name, person.url).as_str());
            }
        }

        text.push('\n');
        text.push('\n');
    }

    text.push_str(&footer);

    let filename = "../../pages/companies.md";
    if let Err(err) = std::fs::write(filename, text) {
        eprintln!("Could not write the file '{filename}': {err}");
    }
}
