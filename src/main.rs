mod regex_word_search;
use regex_word_search::cull_to_pattern;
use std::fs::read;
use std::mem::replace;

struct WordleGame {
    possibilities: Vec<String>,
    confirmed: Vec<(char, usize)>,
    present: Vec<(char, Vec<usize>)>,
    absent: Vec<char>
}

impl WordleGame {
    fn new() -> Self {
        let mut dict_words: Vec<String> = vec![];
        match read("words.txt") {
            Ok(bytes) => {
                let mut word: String = String::new();
                for byte in bytes.iter() {
                    if *byte != b'\n' {
                        if byte.is_ascii() {
                            word.push(*byte as char);
                        }
                    } else {
                        dict_words.push(replace(&mut word, String::new()));
                    }
                }
            }
            Err(_) => {
                println!("Could not read words.txt");
            }
        }   
        WordleGame {
            possibilities: dict_words,
            confirmed: vec![],
            present: vec![],
            absent: vec![]
        }
    }
}

fn main() {
    let mut game: WordleGame = WordleGame::new();
    game.conf
}
