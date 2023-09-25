use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&str]) -> HashSet<&'a str> {
    
    let mut letters = HashSet::new();

    for each_letter in word {
        letters.insert(each_letter);
    }
    
    println!("{:?}", letters);

    letters
}
