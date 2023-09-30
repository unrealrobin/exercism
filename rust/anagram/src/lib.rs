use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    
    let mut anagrams = HashSet::new();

    pub fn sort_the_word(some_word: &str) -> String {
        let mut chars: Vec<char> = some_word.chars().collect();
        chars.sort();
        let sorted_word: String = chars.into_iter().collect();

        sorted_word

    }

    for words in possible_anagrams{
        if words.to_string().to_lowercase() == word.to_string().to_lowercase() {
            continue;
        }
        else if sort_the_word(&words.to_lowercase()) == sort_the_word(&word.to_lowercase()) {
            anagrams.insert(*words);
        }
    }

    anagrams

    
}
