/// Divides the puzzle text into its words. We remove and split at every non-alphabetic
/// character, so "THEY'RE GOOD." gets split into "THEY", "RE" and "GOOD".
pub fn list_words(puzzle: &str) -> Vec<String> {
    let possible_words = puzzle
        .split(|c: char| !c.is_ascii_alphabetic())
        .filter(|s| s.len() > 0);

    possible_words
        .map(|s| s.to_ascii_uppercase().to_string())
        .collect()
}

/// Calculates a word signature of a word.
///
/// We search for identifying information that remains the same after encryption.
/// Example: A word starting with the first two characters equal will also have
/// equal starting characters in their ciphertext (independent of the key).
/// The same is true for every character that is repeated in a word.
///
/// Thus we can calculate a "signature" of a word by giving each unique character
/// in a word its own number:
/// EITHER      -> 0 1 2 3 0 4            with 0=E,1=I,2=T,3=H,4=R
/// NONETHELESS -> 0 1 0 2 3 4 2 5 2 6 6  with 0=N,1=O,2=E,3=T,4=H,5=L,6=S
///
/// The only missing thing is: we ideally want a single number as the signature,
/// since our HashMap in our dictionary works faster with numbers than with
/// strings or else. And since each digit of the signature can range from 0-26,
/// we need to calculate the signature in base_26.
pub fn translate_word_to_signature(word: &str) -> u128 {
    let mut signature = 1; // all our signature start with a 1 in front to prevent 00 == 0 (which is now 100 != 10)
    let mut character_map: [u8; 26] = [99; 26]; // 99 is our "not set" sentinel
    let mut next_map_idx = 0; // our first character gets index 0, then we increment for each unique character that we see

    for character in word.to_ascii_uppercase().chars() {
        if !character.is_ascii_alphabetic() {
            // TODO: for now we require to only input alphabetic characters as words.
            unimplemented!()
        }

        let character_idx = (character as u8) - 65;

        // If we did not encounter this specific character in this word yet,
        // give it a new index number
        if character_map[character_idx as usize] == 99 {
            character_map[character_idx as usize] = next_map_idx;
            next_map_idx += 1;
        }

        // Append the index number of the word to the base_26 number, by shifting
        // all digits to the left and adding the new index.
        //
        // Example in base_10: Word is EITHER and we have already constructed 01230.
        // Now we add the 4 (for the R) to the signature:
        // 01230 * 10 + 4 = 012300 + 4 = 012304
        // The same works with base_26.
        signature = signature * 26 + character_map[character_idx as usize] as u128;
    }
    signature
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_finds_a_valid_signature() {
        assert_eq!(
            translate_word_to_signature("WQELQXNKW"),
            translate_word_to_signature("YESTERDAY")
        );
        assert_eq!(
            translate_word_to_signature("ABACDECFCGG"),
            translate_word_to_signature("NONETHELESS")
        );
        assert_eq!(
            translate_word_to_signature("ABCDAE"),
            translate_word_to_signature("EITHER")
        );
    }
}
