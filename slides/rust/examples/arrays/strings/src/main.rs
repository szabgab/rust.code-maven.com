fn main() {
    let mut animals = ["cat", "snake", "fish"];
    println!("{animals:?}");

    println!("{}", animals[1]);
    animals[1] = "crab";

    println!("{animals:?}");
}
