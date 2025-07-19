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
        add_address(&mut addresses, text);
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

fn add_address(addresses: &mut HashMap<String, Vec<Address>>, text: String) {
    let parts: Vec<&str> = text.split(',').collect();
    let name = parts[0];
    let atype = parts[1];
    let phone = parts[2];

    let address = Address {
        atype: String::from(atype),
        phone: String::from(phone),
    };

    addresses
        .entry(String::from(name))
        .or_default()
        .push(address);
}
