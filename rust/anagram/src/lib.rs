use std::collections::HashSet;

fn ordered_lowercase(input: &str) -> String {
    let mut word_lc: _ = input
        .chars()
        .map(|c| {
            if c.is_lowercase() {
                c.to_string()
            } else {
                c.to_lowercase().to_string()
            }
        })
        .collect::<Vec<String>>();
    word_lc.sort_unstable();

    String::from_iter(word_lc)
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_lc = ordered_lowercase(word);

    let anagrams = possible_anagrams
        .iter()
        .filter(|s| {
            if word.to_lowercase() == s.to_lowercase() {
                return false;
            }

            let s_lc = ordered_lowercase(s);
            let valid_anagram = word_lc == s_lc;

            valid_anagram
        })
        .map(|s| *s);

    HashSet::from_iter(anagrams)
}
