use std::fs::read_to_string;
use std::collections::HashMap;
use std::collections::HashSet;

use crate::utility::char_frequency;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Colour {
    Green,
    Yellow,
    Gray
}

impl Colour {
    pub fn iced(&self) -> iced::Color {
        match self {
            Self::Green => iced::Color::from_rgb8(108, 169, 101),
            Self::Yellow => iced::Color::from_rgb8(200, 182, 83),
            Self::Gray => iced::Color::from_rgb8(120, 124, 127),
        }
    }
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
    known_invalids:  Vec<usize>,
    max_count: usize,
    has_gray: bool
}

impl Default for Letter {
    fn default() -> Self {
        Self {
            in_word: true,
            known_positions: Vec::new(),
            known_invalids: Vec::new(),
            max_count: 5,
            has_gray: false
        }
    }
}

pub struct WordleGame {
    letters: [Letter; 26],
    used_turns: usize,
    words: Vec<String>,
}

impl Default for WordleGame {
    fn default() -> Self {
        Self {
            letters: std::array::from_fn(|_| Letter {
                in_word: true, known_positions: Vec::new(), known_invalids: Vec::new(), max_count: 5, has_gray: false
            }),
            used_turns: 0,
            words: WordleGame::load(),
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

    pub fn add_information(&mut self, word: [char; 5], results: [Colour; 5]) -> Result<(), String> {
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
                    self.letters[i].has_gray = true;
                    if self.letters[i].known_positions.len() == 0 {
                        self.letters[i].in_word = false;
                    } else {
                        self.letters[i].in_word = true;
                        self.letters[i].max_count = self.letters[i].known_positions.len();
                    }
                },
                Colour::Yellow => {
                    self.letters[i].in_word = true;
                    self.letters[i].known_invalids.push(idx);
                },
                Colour::Green => {
                    if self.letters[i].known_invalids.contains(&idx) {
                        return Err(format!("Letter {c} was found in position {idx} but was already eliminated."));
                    }
                    self.letters[i].known_positions.push(idx);
                    if self.letters[i].has_gray {
                        self.letters[i].max_count = self.letters[i].known_positions.len();
                    }
                    self.letters[i].in_word = true;
                }
            }
        }

        for idx in 0..26 {
            let c = num_to_char(idx as u8).unwrap();

            word.iter().enumerate().for_each(|(i, ch)| if *ch == c && results[i] == Colour::Yellow {
                self.letters[idx].in_word = true;
            })
        }

        Ok(())
    }

    fn count_chars(vec: Vec<char>) -> Vec<(char, usize)> {
        let mut map = HashMap::new();
        for c in vec {
            *map.entry(c).or_insert(0) += 1;
        }
        map.into_iter().collect()
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

        if !self.letters.iter().enumerate().map(|(i, l)| {
            if l.known_invalids.len() > 0 && l.in_word {
                word.contains(num_to_char(i as u8).unwrap())
            } else {
                true
            }
        }).all(|b| b) { return false; }

        let counts = Self::count_chars(chars);
        for (letter, max) in counts {
            let idx = char_to_num(letter);
            if max > self.letters[idx as usize].max_count {
                return false;
            }
        }

        is_valid && has_all_known_characters
    }

    pub fn calculate(&self) -> Vec<String> {
        self.words.iter().filter_map(|word| if self.is_match(word) { Some(word.clone()) } else { None }).collect()
    }

    pub fn recommend(&self) -> String {
        // Build a list of unknown characters in the correct order of frequency
        let unknown: Vec<char> = char_frequency(
            self.letters
                .iter()
                .enumerate()
                .filter_map(|(idx, letter)| {
                    if letter.known_invalids.is_empty()
                        && letter.known_positions.is_empty()
                        && letter.in_word
                    {
                        Some(num_to_char(idx as u8).unwrap())
                    } else {
                        None
                    }
                })
                .collect(),
        );

        // Assign weights based on frequency rank (most frequent gets highest score)
        let mut freq_score: HashMap<char, usize> = HashMap::new();
        for (rank, &ch) in unknown.iter().enumerate() {
            freq_score.insert(ch, 26 - rank);
        }

        let words = self.calculate();

        let mut best_word = String::new();
        let mut best_score = 0;
        let mut best_no_dupes = false;

        for word in words {
            let mut seen = HashSet::new();
            let mut has_dupes = false;
            let mut score = 0;

            for c in word.chars() {
                if !seen.insert(c) {
                    has_dupes = true;
                }
                score += freq_score.get(&c).copied().unwrap_or(0);
            }

            if (!has_dupes && (!best_no_dupes || score > best_score))
                || (has_dupes && !best_no_dupes && score > best_score)
            {
                best_word = word;
                best_score = score;
                best_no_dupes = !has_dupes;
            }
        }

        best_word
    }
}
