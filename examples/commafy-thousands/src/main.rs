use thousands::Separable;

fn main() {
    let number = 12345;
    println!("{:10} {:>10}",  number, number.separate_with_commas());

    let number = 123;
    println!("{:10} {:>10}",  number, number.separate_with_commas());

    let number = -12345;
    println!("{:10} {:>10}",  number, number.separate_with_commas());

    let number = -123;
    println!("{:10} {:>10}",  number, number.separate_with_commas());

    let number = -1234.5;
    println!("{:10} {:>10}",  number, number.separate_with_commas());
}
