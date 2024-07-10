fn main() {
    let counter = [0, 0, 0 ,0, 0, 0, 0, 0, 0, 0];
    println!("{} {:?}", counter.len(), counter);

    let counter = [0; 10];
    println!("{} {:?}", counter.len(), counter);


    let answer = [42; 4];
    println!("{} {:?}", answer.len(), answer);

}
