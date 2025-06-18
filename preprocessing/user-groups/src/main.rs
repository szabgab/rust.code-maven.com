use serde::Deserialize;
use std::collections::HashMap;
use thousands::Separable;

#[derive(Debug, Deserialize)]
struct Location {
    city: String,
    state: Option<String>,
    country: String,
}

#[derive(Debug, Deserialize)]
struct Group {
    name: String,
    url: String,
    members: u32,
    location: Location,
    web: Option<String>,
}

fn main() {
    let groups = load_groups();
    user_groups(&groups);
    by_country(&groups);
}

fn load_groups() -> Vec<Group> {
    let filename = std::path::Path::new("../../examples/meetups.yaml");
    let yaml_string = std::fs::read_to_string(filename).unwrap();
    let mut groups: Vec<Group> = match serde_yaml::from_str(&yaml_string) {
        Ok(value) => value,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };
    groups.sort_by(|a, b| b.members.cmp(&a.members));
    groups
}

fn by_country(groups: &[Group]) {
    let mut country_members: HashMap<String, u32> = HashMap::new();
    let mut country_groups: HashMap<String, Vec<&Group>> = HashMap::new();

    for group in groups.iter() {
        let country = group.location.country.clone();
        *country_members.entry(country.clone()).or_insert(0) += group.members;
        country_groups.entry(country).or_default().push(group);
    }

    let mut text = include_str!("header-by-country.md").to_string();
    let mut countries: Vec<_> = country_groups.keys().collect();
    countries.sort();

    for country in countries {
        text.push_str(&format!("\n## {}\n\n", country));
        let mut state_map: HashMap<Option<&String>, Vec<&Group>> = HashMap::new();
        for group in &country_groups[country] {
            state_map
                .entry(group.location.state.as_ref())
                .or_default()
                .push(*group);
        }

        let mut states: Vec<_> = state_map.keys().collect();
        states.sort();

        for state in states {
            if let Some(state_name) = state {
                text.push_str(&format!("### {}\n\n", state_name));
            }
            for group in &state_map[state] {
                let web = match &group.web {
                    Some(url) => format!(" ([web]({}))", url),
                    None => String::new(),
                };
                text.push_str(&format!(
                    "- [{}]({}){} ({} members) - {}\n",
                    group.name,
                    group.url,
                    web,
                    group.members.separate_with_commas(),
                    group.location.city
                ));
            }
            text.push('\n');
        }
    }

    let footer = include_str!("footer.md");
    text.push_str(footer);

    let filename = "../../pages/user-groups-by-country.md";
    if let Err(err) = std::fs::write(filename, text) {
        eprintln!("Could not write the file '{filename}': {err}");
    }
}

fn user_groups(groups: &[Group]) {
    let total = groups.iter().map(|grp| grp.members).sum::<u32>();
    let mut text = format!(
        include_str!("header-user-groups.md"),
        total.separate_with_commas()
    );

    for (ix, group) in groups.iter().enumerate() {
        let web = match &group.web {
            Some(url) => format!("[web]({})", url),
            None => String::new(),
        };
        let location = match &group.location.state {
            Some(state) => format!(
                "{}, {}, {}",
                group.location.city, state, group.location.country
            ),
            None => format!("{}, {}", group.location.city, group.location.country),
        };
        text.push_str(
            format!(
                "| {} | [{}]({}) | {} | {} | {} |\n",
                ix + 1,
                group.name,
                group.url,
                web,
                group.members,
                location
            )
            .as_str(),
        );
    }

    let footer = include_str!("footer.md");
    text.push_str(footer);

    let filename = "../../pages/user-groups.md";
    if let Err(err) = std::fs::write(filename, text) {
        eprintln!("Could not write the file '{filename}': {err}");
    }
}
