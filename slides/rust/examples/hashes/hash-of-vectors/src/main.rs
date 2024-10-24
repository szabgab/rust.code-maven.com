use std::collections::HashMap;

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
struct Address {
    atype: String,
    phone: String,
}

fn main() {
    let mut addresses: HashMap<String, Vec<Address>> = HashMap::new();

    for text in [
        String::from("Foo Bar,home,+03-1234567"),
        String::from("Joe Doe,mobile,+1-1234-567"),
        String::from("Foo Bar,mobile,+42-1234567"),
    ] {
        let parts = text.split(',').collect::<Vec<&str>>();
        let name = parts[0].to_owned();
        let atype = parts[1].to_owned();
        let phone = parts[2].to_owned();

        let addr = Address { atype, phone };

        if !addresses.contains_key(&name) {
            addresses.insert(name.clone(), vec![]);
        }

        addresses.entry(name).and_modify(|value| value.push(addr));
    }

    println!("{:#?}", addresses);
    let expected = HashMap::from([
        (
            String::from("Foo Bar"),
            vec![
                Address {
                    atype: String::from("home"),
                    phone: String::from("+03-1234567"),
                },
                Address {
                    atype: String::from("mobile"),
                    phone: String::from("+42-1234567"),
                },
            ],
        ),
        (
            String::from("Joe Doe"),
            vec![Address {
                atype: String::from("mobile"),
                phone: String::from("+1-1234-567"),
            }],
        ),
    ]);

    assert_eq!(addresses, expected);
}
