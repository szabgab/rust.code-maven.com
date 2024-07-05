fn main() {
    let mut text = String::new();
    println!("ptr: {:p} {:?}", &text, text.as_ptr());

    println!("'{}' len: {} {}", text, text.len(), text.capacity());
    text.push_str("Hello foo, how are you?");
    println!("'{}' len: {} {}", text, text.len(), text.capacity());

    println!("ptr: {:p} {:?}", &text, text.as_ptr());

    //let name = &text[6..9];
    let name = text[6..9].to_owned();
    //println!("'{}'", &text[6..9]);
    println!("name: {name}");
    // //text.insert_str(6, "bar");
    text.replace_range(6..9, "bar");
    // let new_text = text.replace("foo", "bar");
    println!("'{}' len: {} {}", text, text.len(), text.capacity());
    // println!("'{}' len: {} {}", new_text, new_text.len(), new_text.capacity());
    println!("name: {name}");
    println!("ptr: {:p} {:?}", &text, text.as_ptr());

    // text.replace_range(6..9, "abc");
    // // let new_text = text.replace("foo", "bar");
    // println!("'{}' len: {} {}", text, text.len(), text.capacity());


    // let a = text.as_str();
    // let b = &text[..];
    // assert_eq!(a, b);

    // let args = std::env::args().collect::<Vec<String>>();
    // say("hello");
    // let text = String::from("hello");
    // let borrowed_string = &text;    
    // say(borrowed_string);
    // let x: &str = borrowed_string;

    // let text_str = "Hello World";
    // let text_string = text_str.to_owned();
    // let a = &text_string;
    // println!("{text_string}");
    // println!("{a}");
    // assert_eq!(a, &text_string);
    // let b = text_string.as_str();
    // println!("{b}");

    // refref();
}

// fn say(text: &str) {
//     println!("{text}")
// }


// fn refref() {
//     let mut text = String::from("hello");
//     text.extend(["z"]);
//     let text_ref = &text;
    
//     println!("{text_ref}");
// }