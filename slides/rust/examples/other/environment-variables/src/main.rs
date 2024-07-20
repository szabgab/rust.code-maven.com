use std::env;

fn main() {
    for (name, _value) in env::vars() {
        println!("Variable: {name}");
        // match env::var(name) {
        //     Ok(val) => println!("{name}={val}"),
        //     Err(err) => println!("Environment variable {name} does not exist.\n{err}"),
        // }
    }
}
