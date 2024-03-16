use serde::Deserialize;
//use thousands::Separable;

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Address {
    country: String,
    //state: String,
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
    people: Vec<Person>,
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

    let mut text = String::from(
        r#"
---
title: Companies using Rust
timestamp: 2024-03-15T21:30:01
published: false
description: Which company uses Rust with evidence
tags:
    - corporate
    - video
    - article
---

When considering using Rust you will probably want to know who else already uses it. It is usually not easy to get companies tell you if they use Rust or for that matter
any other programming language, but there are various sources we can use to find this information.

"#,
    );

    for company in companies.iter() {
        text.push_str(format!("## [{}]({})\n\n", company.name, company.url,).as_str());
        for address in company.addresses.iter() {
            text.push_str(format!("({}, {})\n", address.country, address.city).as_str());
        }

        text.push('\n');

        for proof in company.proofs.iter() {
            text.push_str(format!("* [{}]({})\n", proof.title, proof.url).as_str());
        }

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
