fn main() {}

fn split_to_strings(sentences: Vec<String>) -> Vec<String> {
    let vector_of_vectors = sentences
        .iter()
        .map(|sentence| {
            sentence
                .split_whitespace()
                .map(|word| word.to_owned())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();
    vector_of_vectors.concat()
}

#[test]
fn test_split() {
    let sentence = String::from("apple banana   peach  pear");
    let words = sentence.split_whitespace().collect::<Vec<&str>>();
    assert_eq!(words, vec!["apple", "banana", "peach", "pear"]);

    let words = sentence
        .split_whitespace()
        .map(|word| word.to_owned())
        .collect::<Vec<String>>();
    assert_eq!(words, vec!["apple", "banana", "peach", "pear"]);

    let vector_of_sentences = vec![
        String::from("apple banana   peach"),
        String::from("pear   cherry"),
    ];
    let words = split_to_strings(vector_of_sentences);
    assert_eq!(words, vec!["apple", "banana", "peach", "pear", "cherry"]);
}
