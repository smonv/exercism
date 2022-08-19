use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let to_lowercase_c = |c: char| {
        if c.is_lowercase() {
            c.to_string()
        } else {
            c.to_lowercase().to_string()
        }
    };

    let words: Vec<_> = word.chars().map(to_lowercase_c).collect();
    let mut word_chars: Vec<_> = word.chars().map(to_lowercase_c).collect();
    word_chars.sort_unstable();

    possible_anagrams
        .iter()
        .filter(|w| {
            let mut w_chars: Vec<_> = w.chars().map(to_lowercase_c).collect();
            let is_same_word = w_chars == words;
            w_chars.sort_unstable();
            word_chars == w_chars && !is_same_word
        })
        .map(|&w| w)
        .collect()
}
