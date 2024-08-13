use regex::Regex;

pub fn cull_to_pattern(regex_expression: &String, possibilities: &mut Vec<String>) -> Result<usize, regex::Error> {
    let rgx: Regex = Regex::new(&regex_expression)?; 
    let mut to_remove: Vec<usize> = vec![];
    for (idx, possibility) in possibilities.iter().enumerate() {
        if rgx.is_match(possibility) {
            to_remove.push(idx);
        }
    }
    for idx in to_remove {
        possibilities.remove(idx);
    }
    Ok(possibilities.len())
}
    
