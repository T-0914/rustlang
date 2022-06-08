// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let magazine_words = count_words(magazine);
    let note_words = count_words(note);

    compare(&magazine_words, &note_words)
}

fn count_words<'a>(sentence: &'a [&str]) -> HashMap<&'a str, i32> {
    let mut words = HashMap::new();
    for (index, word) in sentence.iter().enumerate() {
        let counter = words.entry(*word).or_insert(0);
        *counter += 1;
    }
    words
}

fn compare(first_map: &HashMap<&str, i32>, second_map: &HashMap<&str, i32>) -> bool {
    let is_contain_all_needed_keys = second_map.keys().all(|key| first_map.contains_key(key));

    if !is_contain_all_needed_keys {
        false
    } else {
        second_map
            .keys()
            .all(|&key| first_map.get(key) >= second_map.get(&key))
    }
}
