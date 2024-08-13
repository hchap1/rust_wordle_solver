mod regex_word_search;
use regex_word_search::cull_to_pattern;

fn main() {
    let mut test: Vec<String> = vec![String::from("eggs"), String::from("balls")];
    cull_to_pattern(&String::from("..gs"), &mut test);
}
