macro_rules! make {
    (say hi) => {
        println!("Say hi!")
    };

    (do something) => {
        println!("do something")
    };
}
fn main() {
    make!(say hi);
    make!(do something);
}
