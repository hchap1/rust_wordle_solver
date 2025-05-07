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
    used_turns: usize
}

impl Default for WordleGame {
    fn default() -> Self {
        Self {
            letters: std::array::from_fn(|_| Letter {
                in_word: false, known_positions: Vec::new(), known_invalids: Vec::new()
            }),
            used_turns: 0
        }
    }
}

impl WordleGame {
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

    pub fn is_match(&self, word: &String) -> bool {
        let chars = word.chars().collect::<Vec<char>>();
        word.char_indices().map(|(idx, c)| {
            let i = char_to_num(c) as usize;
            !(self.letters[i].known_invalids.contains(&idx) || !self.letters[i].in_word)
        }).all(|b| b) && self.letters.iter().enumerate().map(|(i, letter)| {
            letter.known_positions.iter().map(|idx| chars[*idx] == num_to_char(i as u8).unwrap()).all(|b| b)
        }).all(|b| b)
    }
}
