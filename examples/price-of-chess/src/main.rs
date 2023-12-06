fn main() {
    let base: u128 = 2;
    let price = base.pow(64) - 1;
    println!("price: {}", price);
    println!("price: {}", commafy(price));

    let weight_in_gram = price as f64 * 0.025;
    println!("weight in gram: {}", weight_in_gram);
    println!("weight in kg: {}", (weight_in_gram / 1000.0).round());
    println!(
        "weight in metric ton: {}",
        (weight_in_gram / 1000.0 / 1000.0).round()
    );
    println!(
        "weight in metric ton: {}",
        commafy((weight_in_gram / 1000.0 / 1000.0).round() as u128)
    );
}

pub fn commafy<Integer: Into<u128> + Copy + std::fmt::Debug + std::fmt::Display>(
    num: Integer,
) -> String {
    let num = format!("{num}");
    let mut ix = 0;
    let num = num
        .chars()
        .rev()
        .map(|chr| {
            ix += 1;
            if ix % 3 == 1 && ix > 1 {
                format!(",{chr}")
            } else {
                format!("{chr}")
            }
        })
        .collect::<String>();
    num.chars().rev().collect::<String>()
}
