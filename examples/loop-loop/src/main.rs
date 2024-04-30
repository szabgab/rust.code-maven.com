use rand::Rng;

fn main() {

    let mut total = 0;

    let result = loop {
        let number = rand::thread_rng().gen_range(1..=10);
        total += number;
        println!("{total}");

        if total > 50 {
            break total;
        }
    };

    println!("The result is {result}");
}
