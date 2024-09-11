use std::collections::HashMap;

use crate::{key::PotentialKey, parsing};

/// How often a word must be mentioned in Google Books. We want to exclude some
/// of the really weird words.
const ALLOWED_WORD_MINPOPULARITY: usize = 100000;

/// Our dictionary maps word signatures to a list of words with that signature.
/// For an explanation of word signatures see parsing::translate_word_to_signature.
pub struct Dictionary(HashMap<u128, Vec<String>>);

impl Dictionary {
    /// Build the signature to words map from our dictionary file.
    pub fn build() -> Self {
        // Parse our dictionary file and extract the words and their usage counts.
        let mut dictionary: HashMap<u128, Vec<String>> = HashMap::default();
        let wordlist = include_str!("dictionary.txt");
        let mut words_and_usage: Vec<(usize, String)> = wordlist
            .lines()
            .map(|s| {
                let mut splits = s.split_whitespace();
                (
                    splits.next().unwrap(),
                    splits.next().unwrap().parse::<usize>().unwrap(),
                    splits.next().unwrap().parse::<usize>().unwrap(),
                )
            })
            .filter(|(_, _, count)| *count >= ALLOWED_WORD_MINPOPULARITY)
            .map(|(s, _, count)| (count, s.to_ascii_uppercase()))
            .collect();

        // Sort by descending usage (most important words first)
        words_and_usage.sort();
        words_and_usage.reverse();

        // Add words to their corresponding signature
        for (_, word) in words_and_usage {
            let signature = parsing::translate_word_to_signature(word.as_str());
            if let Some(entry) = dictionary.get_mut(&signature) {
                entry.push(word.to_string());
            } else {
                dictionary.insert(signature, vec![word.to_string()]);
            }
        }

        Self(dictionary)
    }

    /// Get all possible words from our dictionary. The words must have both, the
    /// correct word signature of the cipherword and they must be matching our
    /// current decryption process (by checking them with the current key).
    pub fn get_possible_words(&self, cipherword: &str, current_key: &PotentialKey) -> Vec<String> {
        let signature = parsing::translate_word_to_signature(cipherword);
        if let Some(entries) = self.0.get(&signature) {
            return entries
                .iter()
                .filter(|&possible_word| current_key.matches(&possible_word, cipherword))
                .cloned()
                .collect();
        }
        vec![]
    }
}
