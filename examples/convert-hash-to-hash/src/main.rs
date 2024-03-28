use std::collections::HashMap;

fn main() {
    with_mutable();
    println!("---------------");
    functional();
    println!("---------------");
    functional_temp();
}

fn with_mutable() {
    let stats: HashMap<&str, usize> = HashMap::from([("red", 10), ("green", 39), ("blue", 23)]);

    let total = stats.values().sum::<usize>();

    let mut perc: HashMap<&str, String> = HashMap::new();
    for field in stats.keys() {
        perc.insert(field, percentage(stats[field], total));
    }

    dbg!(&stats);
    dbg!(total);
    dbg!(perc);
}

fn functional() {
    let stats: HashMap<&str, usize> = HashMap::from([("red", 10), ("green", 39), ("blue", 23)]);

    let total = stats.values().sum::<usize>();

    let perc: HashMap<&&str, String> = HashMap::from_iter(
        stats
            .iter()
            .map(|(k, v)| (k, percentage(*v, total)))
            .collect::<Vec<(&&str, String)>>(),
    );

    dbg!(&stats);
    dbg!(total);
    dbg!(perc);
}

fn functional_temp() {
    let stats: HashMap<&str, usize> = HashMap::from([("red", 10), ("green", 39), ("blue", 23)]);

    let total = stats.values().sum::<usize>();

    let pairs = stats
        .iter()
        .map(|(k, v)| (k, percentage(*v, total)))
        .collect::<Vec<(&&str, String)>>();
    let perc: HashMap<&&str, String> = HashMap::from_iter(pairs);

    dbg!(&stats);
    dbg!(total);
    dbg!(perc);
}

pub fn percentage(num: usize, total: usize) -> String {
    let total = (10000.0 * num as f32 / total as f32).floor();
    (total / 100.0).to_string()
}
