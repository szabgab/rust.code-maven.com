struct CircleOfLife {
    things: Vec<String>,
    index: Option<usize>,
}

impl CircleOfLife {
    fn new(things: &[String]) -> Self {
        CircleOfLife {
            things: things.to_owned(),
            index: None,
        }
    }
}

impl Iterator for CircleOfLife {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index.is_none() {
            self.index = Some(0);
        } else {
            let index = self.index.unwrap();
            self.index = Some(index + 1)
        }

        let index = self.index.unwrap();
        if index >= self.things.len() {
            self.index = None;
            return None;
        }

        Some(self.things[index].clone())
    }
}

fn main() {
    let animals = vec![
        String::from("cat"),
        String::from("dog"),
        String::from("crab"),
    ];

    let mut iterator = CircleOfLife::new(&animals);

    loop {
        if let Some(animal) = iterator.next() {
            println!("{animal}")
        } else {
            println!("No more animals");
            break;
        }
    }

    if let Some(animal) = iterator.next() {
        println!("{animal}")
    } else {
        println!("This iterator is finished");
    }
}
