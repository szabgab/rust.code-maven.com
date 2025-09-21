use liquid_core::{
    Display_filter, Filter, FilterReflection, ParseFilter, Result, Runtime, Value, ValueView,
};
use serde::Serialize;

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(name = "type", description = "Type", parsed(TypeFilter))]
pub struct TypeStr;

#[derive(Debug, Default, Display_filter)]
#[name = "typestr"]
pub struct TypeFilter;

impl Filter for TypeFilter {
    fn evaluate(&self, input: &dyn ValueView, _runtime: &dyn Runtime) -> Result<Value> {
        let keys = ["Presentation", "Break"];
        match input.as_object() {
            Some(obj) => {
                for key in keys {
                    if obj.contains_key(key) {
                        return Ok(Value::scalar(format!("{}", key)));
                    }
                }
                return Ok(Value::scalar("Unknown Item"));
            }
            None => Ok(Value::scalar("Not an object")),
        }
    }
}

#[derive(Serialize)]
struct Presentation {
    title: String,
    length: u32,
    speaker: String,
}

#[derive(Serialize)]
struct Break {
    length: u32,
}

#[derive(Serialize)]
enum Item {
    Presentation(Presentation),
    Break(Break),
}

fn main() {
    let items = get_items();

    let text = items_to_text(&items);
    //println!("{}", text);
    let expected_text = "\
Presentation: Introduction to Rust by Alice (30 mins)
Break: 15 mins
Presentation: Advanced Rust by Bob (45 mins)
";
    assert_eq!(text, expected_text);

    let html = render(&items);
    let expected_html = "\
<ul>
       <li>Introduction to Rust by Alice - (30 minutes)</li>
       <li>Break (15 minutes)</li>
       <li>Advanced Rust by Bob - (45 minutes)</li>
</ul>";
    assert_eq!(html, expected_html);
    //println!("{}", html);
}

fn get_items() -> Vec<Item> {
    vec![
        Item::Presentation(Presentation {
            title: "Introduction to Rust".to_string(),
            length: 30,
            speaker: "Alice".to_string(),
        }),
        Item::Break(Break { length: 15 }),
        Item::Presentation(Presentation {
            title: "Advanced Rust".to_string(),
            length: 45,
            speaker: "Bob".to_string(),
        }),
    ]
}

fn items_to_text(items: &Vec<Item>) -> String {
    let mut result = String::new();
    for item in items {
        match item {
            Item::Presentation(p) => {
                result.push_str(&format!(
                    "Presentation: {} by {} ({} mins)\n",
                    p.title, p.speaker, p.length
                ));
            }
            Item::Break(b) => {
                result.push_str(&format!("Break: {} mins\n", b.length));
            }
        }
    }
    result
}

fn render(items: &Vec<Item>) -> String {
    let template = liquid::ParserBuilder::with_stdlib()
        .filter(TypeStr)
        .build()
        .unwrap()
        .parse_file("templates/page.html")
        .unwrap();

    let globals = liquid::object!({
        "name": "Liquid",
        "items": items,
    });
    let output = template.render(&globals).unwrap();
    output
}
