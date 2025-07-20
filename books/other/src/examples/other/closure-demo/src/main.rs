fn main() {
    do_something();
    do_something_more();
    change_something();
}

fn do_something() {
    let age = 42;

    let display = || println!("The age is {age}");

    display();
}

fn do_something_more() {
    let age = 42;

    let display = || {
        println!("Before");
        println!("More age {age}");
        println!("After");
    };

    display();
}

fn change_something() {
    let mut age = 42;

    let mut change = || {
        age = 43;
    };

    //println!("Age before {age}");
    change();
    println!("Age after {age}");
}
