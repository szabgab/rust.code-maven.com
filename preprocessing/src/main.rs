use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Group {
    name: String,
    url: String,
    members: u32,
    location: String,
}

fn main() {
    let filename = std::path::Path::new("../examples/meetups.yaml");
    let yaml_string = std::fs::read_to_string(filename).unwrap();
    let mut groups: Vec<Group> = match serde_yaml::from_str(&yaml_string) {
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
title: Rust Meetup and user groups
timestamp: 2023-12-11T08:30:01
description: There are many Rust user groups around the world. Here is the full list with location and the number of members.
tags:
    - Meetup
---

I wanted to know the relative sizes of the various Rust-specific Meetup groups so I collected them into a YAML file and then generated this page.
If you are interested which one is meeting in the next month or so check out [This week in Rust](https://this-week-in-rust.org/).
They send out an up-to-date list every week.

| name | members | location |
| ---- | ------- | -------- |
"#,
    );

    groups.sort_by(|a, b| b.members.cmp(&a.members));
    for group in groups {
        text.push_str(
            format!(
                "| [{}]({}) | {} | {} |\n",
                group.name, group.url, group.members, group.location
            )
            .as_str(),
        );
    }

    text.push_str(r#"

For this page even the Markdown file is generated. See the `preprocessing` in the [repository](https://github.com/szabgab/rust.code-maven.com/) and the YAML file.

![](examples/meetups.yaml)
"#);

    let filename = "../pages/meetups.md";
    if let Err(err) = std::fs::write(filename, text) {
        eprintln!("Could not write the file '{filename}': {err}");
    }
}
