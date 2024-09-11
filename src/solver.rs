use crate::dictionary;
use crate::key::PotentialKey;
use crate::parsing;

/// With a given dict and puzzle string, construct a key that maps characters of
/// the ciphertext to possible characters of the plaintext.
///
/// The idea is to iterate over each word of the ciphertext and pull possible
/// words from the dictionary. Then we restrict the keyspace to the characters
/// of those words. We repeat the iteration over the whole puzzle as long as we
/// have changes to the key (since later words can contain hints for earlier
/// words in the puzzle string).
pub fn construct_key(dict: &dictionary::Dictionary, puzzle: &str) -> PotentialKey {
    // Init a key that corresponds to: "every character can be mapped to every
    // other character"
    let mut potential_key = PotentialKey::full();
    let mut previous_key = PotentialKey::default();

    // While the key changes, we iterate over all words.
    while potential_key != previous_key {
        previous_key = potential_key.clone();
        for word in parsing::list_words(puzzle) {
            let possible_words = dict.get_possible_words(word.as_str(), &potential_key);

            // From possible words we generate a key that maps the characters of
            // the cipherword to the characters of the plainwords.
            // Example: XDYY with possible words CALL, TODD and FALL
            // Key: X => [C,F,T], D => [A,O], Y => [L,D]
            let key = PotentialKey::from_possibilities(word.as_str(), &possible_words);

            // Merge the new key with all previous keys
            potential_key = potential_key.merge(&key);
        }
        potential_key.remove_fixed_characters_from_other_mappings();
    }

    potential_key
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puzzle1() {
        let dict = dictionary::Dictionary::build();
        let puzzle = String::from("WQELQXNKW FE AFELUXW, LUIUXXUM FE K IWELQXW, LUNKW FE ZUN'E ZFCL, LAKL'E MAW MQ BKOO FL LAQ VXQEQDL.");
        let key = construct_key(&dict, puzzle.as_str());
        assert_eq!(key.translate(puzzle.as_str()), Ok("YESTERDAY IS HISTORY, TOMORROW IS A MYSTERY, TODAY IS GOD'S GIFT, THAT'S WHY WE CALL IT THE PRESENT.".to_string()));
    }
}
