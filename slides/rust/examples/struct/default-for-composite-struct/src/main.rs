
#[derive(Debug)]
#[allow(dead_code)]
struct Something {
    name: String,
    input: Input,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Input {
    name: String,
}


impl Something {
    pub fn new() -> Something {
        Something {
            name: String::new(),
            input: Input::new(),
        }
    }
}

impl Input {
    pub fn new() -> Input {
        Input {
            name: String::new(),
        }
    }
}


impl Default for Something {
    fn default() -> Something {
        Something {
            name: String::new(),
            input: Input {
                ..Input::default()
            },
        }
    }
}

impl Default for Input {
    fn default() -> Input {
        Input {
            name: String::new(),
        }
    }
}

fn main() {
    let sg = Something {
        name: String::from("Foo Bar"),
        input: Input { name: String::from("input text") },
    };
    dbg!(sg);

    let new = Something::new();
    dbg!(new);

    let empty = Something {
        ..Something::default()
    };
    dbg!(empty);


    let with_name = Something {
        name: String::from("Hello"),
        ..Something::default()
    };
    dbg!(with_name);

}
