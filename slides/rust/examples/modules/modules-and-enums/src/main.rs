
fn main() {
    helper::show_animal();

    let animal = helper::get_animal();
    println!("{:?}", animal);

}


mod helper {

    #[allow(dead_code)]
    #[derive(Debug)]
    pub enum Animal {
        Cat,
        Dog,
    }


    pub fn show_animal() {
        let animal = Animal::Cat;
        println!("{:?}", animal);
    }

    pub fn get_animal() -> Animal {
        Animal::Dog
    }
}