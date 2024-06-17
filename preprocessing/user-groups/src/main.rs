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
    let mut text = format!(include_str!("header.md"), total.separate_with_commas());

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
