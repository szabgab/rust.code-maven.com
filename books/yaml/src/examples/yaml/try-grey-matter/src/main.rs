use gray_matter::Matter;
use gray_matter::engine::YAML;
use serde::Deserialize;


#[derive(Deserialize, Debug)]
struct FrontMatter {
    name: String,
    age: u32,
}

fn main() {
    let data = r#"---
name: Foo
age: 41
```
Text
"#;


    //println!("{data}");

    let matter = Matter::<YAML>::new();
    let result = matter.parse(&data);

    println!("{result:?}");

    println!("{:?}", result.data);
//    let details: FrontMatter = result.data.unwrap().deserialize().unwrap();
//    println!("{details:?}");

}
