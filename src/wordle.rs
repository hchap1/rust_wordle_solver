use std::fs::read_to_string;

pub enum Colour {
    Green,
    Yellow,
    Gray
}

fn num_to_char(number: u8) -> Option<char> {
    if number <= 25 {
        Some((b'a' + number) as char)
    } else {
        None
    }
}

fn char_to_num(character: char) -> u8 {
    character as u8 - b'a'
}

struct Letter {
    in_word: bool,
    known_positions: Vec<usize>,
    known_invalids:  Vec<usize>
}

impl Default for Letter {
    fn default() -> Self {
        Self {
            in_word: true,
            known_positions: Vec::new(),
            known_invalids: Vec::new()
        }
    }
}

pub struct WordleGame {
    letters: [Letter; 26],
    used_turns: usize,
    words: Vec<String>
}

impl Default for WordleGame {
    fn default() -> Self {
        Self {
            letters: std::array::from_fn(|_| Letter {
                in_word: true, known_positions: Vec::new(), known_invalids: Vec::new()
            }),
            used_turns: 0,
            words: WordleGame::load()
        }
    }
}

impl WordleGame {
    pub fn load() -> Vec<String> {
        match read_to_string("wordle_words.txt") {
            Ok(words) => words.lines().map(|line| line.to_string()).collect(),
            Err(_) => panic!("Could not access wordle_words.txt!")
        }
    }

    pub fn cli_information(&mut self, word: &str, results: &str) -> Result<(), String> {
        let chars = word.chars().collect::<Vec<char>>();
        let colours = results.chars().map(|x| match x {
            'g' => Colour::Green,
            'y' => Colour::Yellow,
             _  => Colour::Gray
        }).collect();

        self.add_information(chars, colours)
    }

    pub fn add_information(&mut self, word: Vec<char>, results: Vec<Colour>) -> Result<(), String> {
        self.used_turns += 1;
        for idx in 0..5 {
            let c = match word.get(idx) {
                Some(c) => c,
                None => return Err(String::from("Word did not contain 5 characters."))
            };

            let r = match results.get(idx) {
                Some(r) => r,
                None => return Err(String::from("Result did not contain 5 colours."))
            };

            let i = char_to_num(*c) as usize;

            match r {
                Colour::Gray => {
                    self.letters[i].known_invalids.push(idx);
                    self.letters[i].in_word = false;
                },
                Colour::Yellow => self.letters[i].known_invalids.push(idx),
                Colour::Green => {
                    if self.letters[i].known_invalids.contains(&idx) {
                        return Err(format!("Letter {c} was found in position {idx} but was already eliminated."));
                    }
                    self.letters[i].known_positions.push(idx);
                }
            }
        }
        Ok(())
    }

    pub fn is_match(&self, word: &str) -> bool {
        let chars: Vec<char> = word.chars().collect();
        let is_valid = word.char_indices().map(|(idx, c)| {
            let i = char_to_num(c) as usize;
            let is_valid = !self.letters[i].known_invalids.contains(&idx);
            let in_word = self.letters[i].in_word;
            is_valid && in_word
        }).all(|b| b);

        let mut has_all_known_characters: bool = true;
        'outer: for (idx, letter) in self.letters.iter().enumerate() {
            for known_position in letter.known_positions.iter() {
                if chars[*known_position] != num_to_char(idx as u8).unwrap() {
                    has_all_known_characters = false;
                    break 'outer;
                }
            }
        }

        is_valid && has_all_known_characters
    }

    pub fn calculate(&self) -> Vec<String> {
        self.words.iter().filter_map(|word| if self.is_match(word) { Some(word.clone()) } else { None }).collect()
    }
}
