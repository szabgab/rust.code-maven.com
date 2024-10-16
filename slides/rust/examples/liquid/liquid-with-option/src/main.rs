use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Details {
    weight: u32,
    length: u32,
}

#[derive(Serialize, Deserialize)]
struct Car {
    manufacturer: String,
    color: Option<String>,
    details: Option<Details>,
}

fn main() {
    let template = liquid::ParserBuilder::with_stdlib()
        .build()
        .unwrap()
        .parse(
            "
            Car manufacturer: {{car.manufacturer}}
            Always color: {{ car.color }}
            {% if car.color %}
                Color: {{ car.color }}
            {% else %}
                no color
            {% endif %}

            {% if car.details %}
                Weight: {{ car.details.weight }}
            {% endif %}
        ",
        )
        .unwrap();

    let car = Car {
        manufacturer: String::from("Ford"),
        color: Some(String::from("blue")),
        details: Some(Details {
            weight: 1000,
            length: 400,
        }),
    };

    let globals = liquid::object!(
    {
        "car": car,
    });
    let output_1 = template.render(&globals).unwrap();

    println!("{output_1}");

    let car = Car {
        manufacturer: String::from("Ford"),
        color: None,
        details: None,
    };

    let globals = liquid::object!(
    {
        "car": car,
    });
    let output_2 = template.render(&globals).unwrap();

    println!("{output_2}");


    // verify
    let output = output_1 + &output_2;
    let expected = std::fs::read_to_string("out.txt").unwrap();
    assert_eq!(output.trim(), expected.trim());
}
