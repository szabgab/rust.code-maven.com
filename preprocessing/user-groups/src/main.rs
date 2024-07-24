use serde::Deserialize;
use thousands::Separable;

#[derive(Debug, Deserialize)]
struct Group {
    name: String,
    url: String,
    members: u32,
    location: String,
    web: Option<String>,
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
    let mut text = format!(include_str!("header.md"), total.separate_with_commas());

    groups.sort_by(|a, b| b.members.cmp(&a.members));
    for (ix, group) in groups.iter().enumerate() {
        let web = match &group.web {
            Some(url) => format!("[web]({})", url),
            None => String::new(),
        };
        text.push_str(
            format!(
                "| {} | [{}]({}) | {} | {} | {} |\n",
                ix + 1,
                group.name,
                group.url,
                web,
                group.members,
                group.location
            )
            .as_str(),
        );
    }

    let footer  = include_str!("footer.md");
    text.push_str(&footer);

    let filename = "../../pages/user-groups.md";
    if let Err(err) = std::fs::write(filename, text) {
        eprintln!("Could not write the file '{filename}': {err}");
    }
}
