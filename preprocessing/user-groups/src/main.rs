use serde::Deserialize;
use thousands::Separable;

#[derive(Debug, Deserialize)]
struct Group {
    name: String,
    url: String,
    members: u32,
    location: String,
}

fn main() {
    meetups();
}

fn meetups() {
    let filename = std::path::Path::new("../../examples/meetups.yaml");
    let yaml_string = std::fs::read_to_string(filename).unwrap();
    let mut groups: Vec<Group> = match serde_yaml::from_str(&yaml_string) {
        Ok(value) => value,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };
    //println!("{:?}", data);

    let total = groups.iter().map(|grp| grp.members).sum::<u32>();
    let mut text = format!(
        r#"
---
title: Rust Meetup and user groups
timestamp: 2023-12-11T08:30:01
published: true
description: There are many Rust user groups around the world. Here is the full list with location and the number of members.
tags:
    - Meetup
    - Stats
---

I wanted to know the relative sizes of the various Rust-specific Meetup groups so I collected them into a YAML file and then generated this page.
If you are interested which one is meeting in the next month or so check out [This week in Rust](https://this-week-in-rust.org/).
They send out an up-to-date list every week.

If you find a group near-by where you live or where you work, join them. If none of them are near-by, don't worry, there are quite a few that have
online meetings you can join from any place.

Another measurement could be frequency of events and the number of attendees at the events, but it that's much harder to collect so I stayed with
the number of members.

* On 2023.12.11 there were 99 groups with a total of 59,629 members.
* On 2024.03.17 there were 113 groups with a total of 65,200 members.

Total number of members: {} (the same person might be member of several groups.)

| # | name | members | location |
| - | ---- | ------- | -------- |
"#, total.separate_with_commas()
    );

    groups.sort_by(|a, b| b.members.cmp(&a.members));
    for (ix, group) in groups.iter().enumerate() {
        text.push_str(
            format!(
                "| {} | [{}]({}) | {} | {} |\n",
                ix + 1,
                group.name,
                group.url,
                group.members,
                group.location
            )
            .as_str(),
        );
    }

    text.push_str(r#"

For this page even the Markdown file is generated. See the `preprocessing` in the [repository](https://github.com/szabgab/rust.code-maven.com/) and the YAML file.

"#);

    let filename = "../../pages/user-groups.md";
    if let Err(err) = std::fs::write(filename, text) {
        eprintln!("Could not write the file '{filename}': {err}");
    }
}
