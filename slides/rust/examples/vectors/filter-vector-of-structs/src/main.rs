#[derive(Debug)]
#[allow(dead_code)]
struct Something {
    number: i32,
    text: String,
    numbers: Vec<i32>,
}

fn main() {
    let va: Vec<Something> = vec![
        Something {
            number: 1,
            text: String::from("small"),
            numbers: vec![1, 2, 3],
        },
        Something {
            number: 11,
            text: String::from("medium"),
            numbers: vec![11, 12],
        },
        Something {
            number: 101,
            text: String::from("large"),
            numbers: vec![101],
        },
    ];
    println!("{:#?}", va);

    //let vb = &va.iter().collect::<Vec<&Something>>();
    let vb = &va.iter().collect::<Vec<_>>();

    let v_big = &vb
        .iter()
        .filter(|thing| thing.number > 20)
        .collect::<Vec<_>>();
    //let v_big = &vb.into_iter().filter(|thing| thing.number > 20).collect::<Vec<_>>();
    println!("{:#?}", v_big);
    println!("{:#?}", vb);
}
