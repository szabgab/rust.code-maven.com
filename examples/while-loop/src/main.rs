use rand::Rng;

fn main() {

    let mut total = 0;

    while total < 50 {
        let number = rand::thread_rng().gen_range(1..=10);
        total += number;
        println!("{total}");
    }
}
