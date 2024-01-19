
use std::fs::File;
use std::io::Write;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let name = &args[1];
    println!("Hello {name}");
}

fn add(x: i32, y: i32) -> i32 {
    let time: u64 = rand::random();
    let time = time % 3;
    
    std::thread::sleep(std::time::Duration::from_secs(time));

    match std::env::var("RESULT_PATH") {
        Ok(file_path) => {
            let mut file = File::create(&file_path).unwrap();
            println!("add({x}, {y}) file {file_path}");
            writeln!(&mut file, "{}", x + y).unwrap();        
        },
        Err(_) => {},
    };
    x + y
}


#[test]
fn test_add_2_3_5() {
    let tmp_dir = tempfile::tempdir().unwrap();
    println!("tmp_dir: {:?}", tmp_dir);
    let file_path = tmp_dir.path().join("result.txt");
    println!("2+3 - file_path {:?}", file_path);
    std::env::set_var("RESULT_PATH", &file_path);

    let result= add(2, 3);
    assert_eq!(result, 5);


    let result = std::fs::read_to_string(file_path).unwrap();
    assert_eq!(result, "5\n");
}

#[test]
fn test_add_4_4_8() {
    let tmp_dir = tempfile::tempdir().unwrap();
    println!("tmp_dir: {:?}", tmp_dir);
    let file_path = tmp_dir.path().join("result.txt");
    println!("4+4 - file_path {:?}", file_path);
    std::env::set_var("RESULT_PATH", &file_path);

    let result= add(4, 4);
    assert_eq!(result, 8);

    let result = std::fs::read_to_string(file_path).unwrap();
    assert_eq!(result, "8\n");
}
