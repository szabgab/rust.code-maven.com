struct Person {
    name: String,
    count: i32,
}

// #[test]
// fn test_hello_world() {
//     //insta::assert_debug_snapshot!(vec![1, 2, 3]);
//     insta::assert_snapshot!("Hello", @"Hello");
// }

fn create_person(name: String, count: i32) -> Person {
    Person {
        name, count
    }
}

#[test]
fn test_person() {
    let alice = create_person(String::from("Alice"), 23);


    insta::assert_snapshot!(alice, @Person);
    
    //insta::assert_debug_snapshot!(vec![1, 2, 3]);
    // insta::assert_snapshot!("Hello", @"Hello");
}

//use similar::{ChangeTag, TextDiff};

fn main() {
    // let diff = TextDiff::from_lines(
    //     "Hello World\nThis is the second line.\nThis is the third.",
    //     "Hallo Welt\nThis is the second line.\nThis is life.\nMoar and more",
    // );

    // for change in diff.iter_all_changes() {
    //     let sign = match change.tag() {
    //         ChangeTag::Delete => "-",
    //         ChangeTag::Insert => "+",
    //         ChangeTag::Equal => " ",
    //     };
    //     print!("{}{}", sign, change);
    // }
}


// use daft::Diffable;

// #[derive(Diffable)]
// struct MyStruct {
//     a: i32,
//     b: String,
// }

// fn main() {

//     let before = MyStruct {
//         a: 1,
//         b: "hello".to_owned(),
//     };
//     let after = MyStruct {
//         a: 2,
//         b: "world".to_owned(),
//     };


//     // You can diff them like so:
//     let diff = before.diff(&after);
//     println!("{:?}", diff);

//     // And compare the results:
//     assert_eq!(*diff.a.before, 1);
//     assert_eq!(*diff.a.after, 2);
//     assert_eq!(diff.b.before, "hello");
//     assert_eq!(diff.b.after, "world");
// }

// use structdiff::{Difference, StructDiff};

// #[derive(Debug, PartialEq, Clone, Difference)]
// struct Example {
//     field1: f64,
//     #[difference(skip)]
//     field2: Vec<i32>,
//     // #[difference(collection_strategy="unordered_array_like")]
//     // field3: BTreeSet<usize>,
// }

// fn main() {
    
//     let x = 2;
//     let y = 3;
//     let z: i8 = x +y;
    
//     let first = Example {
//         field1: 0.0,
//         field2: vec![],
// //        field3: vec![1, 2, 3].into_iter().collect(),
//     };

//     let second = Example {
//         field1: 3.14,
//         field2: vec![1],
// //        field3: vec![2, 3, 4].into_iter().collect(),
//     };

//     let diffs = first.diff(&second);
//     // diffs is now a Vec of differences between the two instances,
//     // with length equal to number of changed/unskipped fields
//     assert_eq!(diffs.len(), 2);
//     println!({:?}, diffs[0])

// //     let diffed = first.apply(diffs);
// //     // diffed is now equal to second, except for skipped field
// //     assert_eq!(diffed.field1, second.field1);
// // //    assert_eq!(&diffed.field3, &second.field3);
// //     assert_ne!(diffed, second);
// }
