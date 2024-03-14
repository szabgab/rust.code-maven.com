
fn main() {
    let res = guess(42);
    assert_eq!(res, Ok("hit"));
    assert_eq!(res.unwrap(), "hit");

    let res = guess(50);
    assert!(res.is_ok());
    assert_eq!(res, Ok("Too big"));

    let res = guess(0);
    assert!(res.is_ok());
    assert_eq!(res, Ok("Too small"));


    assert!(guess(150).is_err());
    assert_eq!(guess(150), Err(String::from("Out of range: 150 is too big")));

    assert!(guess(-150).is_err());
    assert_eq!(guess(-150), Err(String::from("Out of range: -150 is too small")));
}

fn guess(num: i32) -> Result<&'static str, String> {
    let hidden_number = 42;

    if num < -100 {
        return Err(format!("Out of range: {num} is too small"));
    }
    if num > 100 {
        return Err(format!("Out of range: {num} is too big"));
    }
    if num < hidden_number {
        return Ok("Too small");
    }
    if num > hidden_number {
        return Ok("Too big");
    }

    Ok("hit")
}
