#![allow(dead_code)]

fn compute(input: bool) -> Option<String> {
    if input {
        Some(String::from("text"))
    } else {
        None
    }
}

fn manual(input: bool) -> String {
    let result = compute(input);

    if result.is_none() {
        return String::from("Missing");
    }
    let data = result.unwrap();

    // process data:
    format!("Process {data}")
}

fn with_match(input: bool) -> String {
    let result = compute(input);

    match result {
        None => String::from("Missing"),
        Some(data) => {
            // process data:
            format!("Process {data}")
        }
    }
}

fn with_match_and(input: bool) -> String {
    let result = compute(input);

    let data = match result {
        None => return String::from("Missing"),
        Some(data) => data,
    };

    // process data:
    format!("Process {data}")
}

macro_rules! ok_or_return {
    ($cond: expr, $result: expr) => {
        match $cond {
            None => return $result,
            Some(data) => data,
        }
    };
}

fn with_macro(input: bool) -> String {
    let result = compute(input);

    let data = ok_or_return!(result, String::from("Missing"));

    // process data:
    format!("Process {data}")
}

fn let_else(input: bool) -> String {
    let result = compute(input);

    let Some(data) = result else {
        return String::from("Missing");
    };

    // process data:
    format!("Process {data}")
}

#[test]
fn test_compute() {
    assert_eq!(compute(true), Some(String::from("text")));
    assert_eq!(compute(false), None);
}

#[test]
fn test_manual() {
    assert_eq!(manual(true), String::from("Process text"));
    assert_eq!(manual(false), String::from("Missing"));
}

#[test]
fn test_with_match() {
    assert_eq!(with_match(true), String::from("Process text"));
    assert_eq!(with_match(false), String::from("Missing"));
}

#[test]
fn test_with_match_and() {
    assert_eq!(with_match_and(true), String::from("Process text"));
    assert_eq!(with_match_and(false), String::from("Missing"));
}


#[test]
fn test_with_macro() {
    assert_eq!(with_match_and(true), String::from("Process text"));
    assert_eq!(with_match_and(false), String::from("Missing"));
}

#[test]
fn test_let_else() {
    assert_eq!(let_else(true), String::from("Process text"));
    assert_eq!(let_else(false), String::from("Missing"));
}
