macro_rules! prt {
    ($var: expr) => {
        println!(
            "{:p} {:?} {} {} {}",
            &$var,
            $var.as_ptr(),
            $var.len(),
            $var.capacity(),
            $var
        );
    };
}
fn main() {
    let animals = vec![
        String::from("cat"),
        String::from("dog"),
        String::from("fish"),
        String::from("crab"),
        String::from("mouse"),
        String::from("moose"),
    ];

    println!(
        "{}",
        animals.iter().all(|animal| {
            prt!(animal);
            !animal.is_empty()
        })
    );
    println!();

    // println!("{}", animals.into_iter().all(|animal| {
    //     prt!(animal);
    //     animal.len() > 0
    // }));

    // // This is like into_iter, it moves the animals
    // for animal in animals {
    //     prt!(animal);
    // }

    // pluck out two values and take ownership of them so we
    // won't be able to use animals any more.
    // let short = animals.into_iter().take(2).collect::<Vec<_>>();
    // for animal in short {
    //     prt!(animal);
    // }
    // println!();

    let short = animals.iter().take(2).clone().collect::<Vec<_>>();
    for animal in short {
        prt!(animal);
    }
    println!();

    let short = animals
        .iter()
        .take(2)
        .map(|thing| {
            println!("cloning");
            thing.clone()
        })
        .collect::<Vec<_>>();
    for animal in short {
        prt!(animal);
    }
    println!();

    let short = animals.iter().take(2).cloned().collect::<Vec<_>>();
    for animal in short {
        prt!(animal);
    }
    println!();

    for animal in animals {
        prt!(animal);
    }
}
