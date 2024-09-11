use std::collections::HashSet;
use std::fmt::Display;

/// A potential key is a mapping of each of the 26 ciphertext characters to
/// a set of possible plaintext characters.
/// So E=>[A,B,C] means that a E in the ciphertext might be decrypted to either
/// an A, B or C.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PotentialKey([HashSet<char>; 26]);

impl TryFrom<&str> for PotentialKey {
    type Error = &'static str;

    /// The decipher website has the correct keys written as strings in their
    /// source code. This function can take this string and return a proper
    /// key object from that.
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.chars().count() != 26 {
            return Err("Wrong character count during key construction.");
        }

        let mut key = Self::default();
        for (idx, character) in value.char_indices() {
            if !character.is_ascii_alphabetic() {
                return Err("Invalid character during key construction.");
            }
            let code = (character.to_ascii_uppercase() as u8) - 65;
            key.0[code as usize].insert((idx as u8 + 65) as char);
        }
        Ok(key)
    }
}

impl Default for PotentialKey {
    /// Returns an empty key where every mapping is empty.
    fn default() -> Self {
        Self(Default::default())
    }
}

impl Display for PotentialKey {
    /// Returns a printable representation of the current potential key for
    /// display purposes.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (idx, chars) in self.0.iter().enumerate() {
            write!(f, "{} => [", (idx as u8 + 65) as char)?;
            let mut sorted_chars: Vec<char> = chars.clone().into_iter().collect();
            sorted_chars.sort();
            for kchar in sorted_chars {
                write!(f, "{},", kchar)?;
            }
            writeln!(f, "] ")?;
        }
        write!(f, "")
    }
}

impl PotentialKey {
    /// Returns a "full" key, aka a key where no character is decided yet.  So
    /// each ciphertext character maps to the 25 other characters as plaintext
    /// characters.  Since we exclude mappings to themselves (A->A), we skip the
    /// identical character.
    pub fn full() -> PotentialKey {
        let mut key = Self::default();
        for (idx, entry) in key.0.iter_mut().enumerate() {
            for i in 0..26 {
                if i != idx {
                    entry.insert((i as u8 + 65) as char);
                }
            }
        }
        key
    }

    /// Construct a potential key from a list of possible words for a ciphertext.
    ///
    /// This basically iterates over the characters of the cipherword and
    /// adds each character of the possible words to the mapping. Characters
    /// that are not in the cipherword will get the "full" mapping, since we
    /// cannot say anything about them.
    ///
    /// Example:
    /// Cipherword | Possible word 1 | Possible word 2 | Resulting mapping
    /// X            C                 T                 X => [C,T]
    /// Y            A                 R                 Y => [A,R]
    /// Z            L                 E                 Z => [L,E]
    /// Z            L                 E                 Z => [L,E]
    pub fn from_possibilities(cipherword: &str, possible_words: &[String]) -> PotentialKey {
        let mut key = Self::default();
        for possible_word in possible_words {
            for (cwc, pwc) in cipherword.chars().zip(possible_word.chars()) {
                let cwc_idx = cwc as u8 - 65;
                key.0[cwc_idx as usize].insert(pwc);
            }
        }
        for (idx, entry) in key.0.iter_mut().enumerate() {
            if entry.len() == 0 {
                for i in 0..26 {
                    if i != idx {
                        entry.insert((i as u8 + 65) as char);
                    }
                }
            }
        }
        key
    }

    /// Merge two potential keys.
    ///
    /// This will calculate the intersection of mappings in the two keys.
    /// Example:
    /// We know from a word in a puzzle that E => [A,B,C,D].
    /// We know from another word in a puzzle that E => [C,D,F,G].
    /// Then we know from both that E => [C,D] since this is the intersection of
    /// both.
    pub fn merge(&self, other: &Self) -> Self {
        let mut result = Self::default();

        for (idx, (self_possibilties, other_possibilities)) in
            self.0.iter().zip(other.0.iter()).enumerate()
        {
            result.0[idx] = self_possibilties
                .intersection(other_possibilities)
                .copied()
                .collect();
        }

        result
    }

    /// Remove characters that we already decoded from the other mappings in the key.
    ///
    /// If we have a mapping with only one character, for example A => [B], then
    /// we know that no other character can map to B anymore. Thus we can remove
    /// B from the mappings of all other characters.
    pub fn remove_fixed_characters_from_other_mappings(&mut self) {
        let mut remove_candidates: Vec<(usize, char)> = vec![];
        for (idx, chars) in self.0.iter().enumerate() {
            if chars.len() == 1 {
                remove_candidates.push((idx, *chars.iter().last().unwrap()));
            }
        }

        for (idx, rchar) in remove_candidates {
            for (kidx, kchars) in self.0.iter_mut().enumerate() {
                if idx == kidx {
                    continue;
                }

                kchars.remove(&rchar);
            }
        }
    }

    /// Check if a given cipherword can decode to a given plaintext word under
    /// the current key.
    pub fn matches(&self, plainword: &str, cipherword: &str) -> bool {
        cipherword.char_indices().all(|(idx, c)| {
            self.0[(c as u8 - 65) as usize].contains(&plainword.chars().nth(idx).unwrap())
        })
    }

    /// Decrypt the input string with the current key. If we still have
    /// multiple options for a character, then we will decrypt to a '*' to mark
    /// such.
    pub fn translate(&self, input: &str) -> Result<String, &'static str> {
        let mut result = String::new();
        for character in input.to_ascii_uppercase().chars() {
            if !character.is_ascii_alphabetic() {
                result.push(character);
                continue;
            }
            match character.to_ascii_uppercase() {
                'A'..='Z' => {
                    let idx = (character as u8) - 65;
                    let translated_key = self.0.get(idx as usize).unwrap();
                    if translated_key.len() == 0 {
                        return Err("Key contains no solutions for this character.");
                    } else if translated_key.len() > 1 {
                        result.push('*');
                    } else if let Some(&translated_character) = translated_key.iter().last() {
                        result.push(translated_character);
                    }
                }
                _ => unimplemented!(),
            }
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_translates_correctly() {
        let puzzle = String::from("WQELQXNKW FE AFELUXW, LUIUXXUM FE K IWELQXW, LUNKW FE ZUN'E ZFCL, LAKL'E MAW MQ BKOO FL LAQ VXQEQDL.");
        let key = PotentialKey::try_from("KJBNQCZAFYSOIDUVRXELHPMGWT").unwrap();
        assert_eq!(key.translate(puzzle.as_str()), Ok("YESTERDAY IS HISTORY, TOMORROW IS A MYSTERY, TODAY IS GOD'S GIFT, THAT'S WHY WE CALL IT THE PRESENT.".to_string()));
    }
}
