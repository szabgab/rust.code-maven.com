use serde::Deserialize;

#[allow(unused)]
#[derive(Debug, Deserialize, PartialEq)]
struct Person {
    name: String,
    phone: String,
}

fn main() {
    load_data_to_person();
    parse_to_phone();
    load_data_to_deep_person();
}

fn load_data_to_person() {
    let path = "data.json";
    let content = std::fs::read_to_string(path).unwrap();
    let person = serde_json::from_str::<Person>(&content).unwrap();
    println!("person {:?}", person);
    assert_eq!(
        person,
        Person {
            name: String::from("Mr. Plow"),
            phone: String::from("636-555-3226")
        }
    );
}

fn parse_to_phone() {
    let content = r#"{"area": "123", "number": "456-789"}"#.to_string();
    let phone = serde_json::from_str::<Phone>(&content).unwrap();
    println!("phone {:?}", phone);
    assert_eq!(
        phone,
        Phone {
            area: String::from("123"),
            number: String::from("456-789")
        }
    );
}

fn load_data_to_deep_person() {
    let path = "data.json";
    let content = std::fs::read_to_string(path).unwrap();
    let deep_person = serde_json::from_str::<DeepPerson>(&content).unwrap();
    println!("deep_person {:?}", deep_person);
    assert_eq!(
        deep_person,
        DeepPerson {
            name: String::from("Mr. Plow"),
            phone: Phone {
                area: String::from("636"),
                number: String::from("555-3226")
            }
        }
    );
}

#[allow(unused)]
#[derive(Debug, Deserialize, PartialEq)]
struct Phone {
    area: String,
    number: String,
}

#[allow(unused)]
#[derive(Debug, Deserialize, PartialEq)]
struct DeepPerson {
    name: String,

    #[serde(deserialize_with = "from_full_phone")]
    phone: Phone,
}

use serde::de;

fn from_full_phone<'de, D>(deserializer: D) -> Result<Phone, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    //println!("s {:?}", s);
    let (area, number) = s
        .split_once('-')
        .ok_or(de::Error::custom("invalid phone"))?;

    let p = Phone {
        area: area.to_owned(),
        number: number.to_owned(),
    };
    //println!("phone {:?}", p);
    Ok(p)
}
