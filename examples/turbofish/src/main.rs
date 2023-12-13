fn main() {
    parse();
    split();
}

fn parse() {
    let input = "41";

    let val: i32 = input.parse().unwrap();
    println!("next number is {}", val+1);

    let val = input.parse::<i32>().unwrap();
    println!("next number is {}", val+1);

    // type annotation needed
    // let val = input.parse::<_>().unwrap();
    // println!("next number is {}", val+1);
}

fn split() {
    let text = "apple,banana,peach,nut";

    let parts: Vec<&str> = text.split(',').collect();
    println!("{:?}", parts);

    let parts = text.split(',').collect::<Vec<&str>>();
    println!("{:?}", parts);

    let parts: Vec<_> = text.split(',').collect();
    println!("{:?}", parts);

    let parts = text.split(',').collect::<Vec<_>>();
    println!("{:?}", parts);

    // type annotation needed
    // let parts = text.split(',').collect();
    // println!("{:?}", parts);

}