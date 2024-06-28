use colored::Colorize;

fn main() {
    let text = "Hello, world!";
    println!("{}", text);
    println!("{}", text.red());
    println!("{}", text.red().bold());
    println!("{}", text.blue().bold());
    println!("{}", text.truecolor(0, 136, 136));
    println!("{}", text.white().bold().on_black());
}
