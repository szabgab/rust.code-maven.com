use serde::Deserialize;
//use thousands::Separable;

// TODO: separate pages by language
// TODO: separate pages by country
// TODO: order by date


#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
struct Sponsored {
    name: String,
    github: String,
    website: Option<String>,
    article: Option<String>,
    organization: bool,
}

fn main() {
    sponsored();
}


fn sponsored() {
    let filename = std::path::Path::new("../../examples/sponsoring.yaml");
    let yaml_string = std::fs::read_to_string(filename).unwrap();
    let sponsored: Vec<Sponsored> = match serde_yaml::from_str(&yaml_string) {
        Ok(value) => value,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    generate_md_file(&sponsored);
}

fn generate_md_file(sponsored: &[Sponsored]) {
    let mut text = String::from(include_str!("header.md"));

    for sp in sponsored.iter() {
        text.push_str(format!("## [{}](https://github.com/sponsors/{})\n\n",  sp.name, sp.github).as_str());
        if let Some(website) = &sp.website {
            text.push_str(format!("[website]({})\n\n",  website).as_str());
        }
        if let Some(article) = &sp.article {
            text.push_str(format!("[article]({})\n\n",  article).as_str());
        }

        text.push('\n');
        text.push('\n');
        text.push('\n');
    }

    text.push_str(r#"
    
    For this page even the Markdown file is generated. See the `preprocessing/sponsoring` in the [repository](https://github.com/szabgab/rust.code-maven.com/) and the YAML file.    
    "#);

    let filename = "../../pages/sponsoring.md";
    if let Err(err) = std::fs::write(filename, text) {
        eprintln!("Could not write the file '{filename}': {err}");
    }
}
