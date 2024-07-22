#![allow(dead_code, unused_variables)]



struct HasDrop {
    name: String,
}

impl Drop for HasDrop {
    fn drop(&mut self) {
        println!("Dropping HasDrop! {}", self.name);
    }
}

struct HasTwoDrops {
    one: HasDrop,
    two: HasDrop,
}

impl Drop for HasTwoDrops {
    fn drop(&mut self) {
        println!("Dropping HasTwoDrops!");
    }
}


fn main() {
    let hd = HasDrop {
        name: String::from("Foo"),
    };
    let _hd = HasTwoDrops {
        one: HasDrop {
            name: String::from("first"),
        },
        two: HasDrop {
            name: String::from("second"),
        },
    };

    calc(3, 0);
}



fn calc(x: i32, y: i32) -> i32 {
    if y == 0 {
        panic!("oups");
    }
    x / y
}
