use rand::Rng;
fn main() {
    let mut number = 0;
    let last_number = loop {
        let random_number = rand::thread_rng().gen_range(1..=20);
        number += random_number;
        println!("{number}");
        if number > 100 {
            break random_number;
        }
    };

    println!();
    println!("{number} {last_number}");
}
