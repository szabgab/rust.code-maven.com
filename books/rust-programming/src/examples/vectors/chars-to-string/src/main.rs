fn main() {
    let letters = vec!['R', 'u', 's', 't'];
    let name = letters.iter().collect::<String>();
    println!("{name}");

    let name = letters.into_iter().collect::<String>();
    println!("{name}");

    let text = String::from("The black cat");
    println!("{:?}", text.chars());
    let chars = text.chars().collect::<Vec<char>>();
    println!("{chars:?}");
    let text2 = text.chars().collect::<String>();
    assert_eq!(text, text2);

    let reversed: String = text.chars().rev().collect();
    println!("{text}");
    println!("{reversed}");
}
