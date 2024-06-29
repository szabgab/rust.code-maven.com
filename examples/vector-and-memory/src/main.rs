macro_rules! prt {
    ($vector: expr) => {
        println!("len: {:2} capacity: {:2}, addr: {:p} ptr: {:?}", $vector.len(), $vector.capacity(), &$vector, $vector.as_ptr());    
    };
}

macro_rules! prt_array {
    ($vector: expr) => {
        println!("len: {:2} capacity: --, addr: {:p} ptr: {:?}", $vector.len(), &$vector, $vector.as_ptr());    
    };
}

fn main() {
    let mut names: Vec<String> = vec![];
    prt!(names);

    names.push(String::from("foo"));
    prt!(names);
    println!("{:p}", &names[0]);

    let other = vec![String::from("another one")];
    prt!(other);


    for i in 1..10 {
        names.push(format!("foo {i}"));
        prt!(names);
    }
    //println!("{:p}", &names[0]);
    pass_array(&names);
    pass_vector(names);
}

fn pass_vector(values: Vec<String>) {
    prt!(values);
}

fn pass_array(values: &[String]) {
    prt_array!(values);
}