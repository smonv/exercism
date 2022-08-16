// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let magazine_words = magazine
        .iter()
        .fold(HashMap::<&str, i8>::new(), |mut words, str| {
            words.entry(*str).and_modify(|x| *x += 1).or_insert(1);
            words
        });

    let note_words = note
        .iter()
        .fold(HashMap::<&str, i8>::new(), |mut words, str| {
            words.entry(*str).and_modify(|x| *x += 1).or_insert(1);
            words
        });

    note_words
        .iter()
        .all(|(word, count)| magazine_words.get(word).unwrap_or(&0) >= count)
}
