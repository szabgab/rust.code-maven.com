use rusty_paseto::prelude::*;

fn main() {

    // create a key specifying the PASETO version and purpose
    let key = PasetoSymmetricKey::<V4, Local>::from(Key::from(b"wubbalubbadubdubwubbalubbadubdub"));

    // use a default token builder with the same PASETO version and purpose
    let token = PasetoBuilder::<V4, Local>::default().build(&key).unwrap();
    println!("{token}");
}
