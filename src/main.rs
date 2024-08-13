mod regex_word_search;
use regex_word_search::cull_to_pattern;
use std::fs::read;

struct WordleGame {
    possibilities: Vec<String>,
    confirmed: Vec<(char, usize)>,
    present: Vec<(char, Vec<usize>)>,
    absent: Vec<char>
}

impl WordleGame {
    fn new() -> Self {
        read("dictionary.txt")

fn main() {
    let mut test: Vec<String> = vec![String::from("eggs"), String::from("balls")];
    match cull_to_pattern(&String::from("..gs"), &mut test) {
        Ok(_) => {
            for word in test {
                println!("{word}");
            }
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
}
