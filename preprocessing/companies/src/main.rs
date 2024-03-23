use serde::Deserialize;
//use thousands::Separable;

// TODO: separate pages by language
// TODO: separate pages by country
// TODO: order by date

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Address {
    country: String,
    //state: String,
    #[serde(default = "get_empty_string")]
    city: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Person {
    name: String,
    url: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Proof {
    url: String,
    title: String,
    ptype: String,
    language: String,
    date: String,
    people: Vec<Person>,
    description: Option<String>,
}

#[derive(Debug, Deserialize)]
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

    generate_md_file(&companies);
}

fn generate_md_file(companies: &[Company]) {
    let mut text = String::from(include_str!("header.md"));

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
                    text.push('\n');
                    text.push_str(description);
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

    text.push_str(r#"
    
    For this page even the Markdown file is generated. See the `preprocessing/companies` in the [repository](https://github.com/szabgab/rust.code-maven.com/) and the YAML file.
    
    {% include file="examples/companies.yaml" %}
    "#);

    let filename = "../../pages/companies.md";
    if let Err(err) = std::fs::write(filename, text) {
        eprintln!("Could not write the file '{filename}': {err}");
    }
}
