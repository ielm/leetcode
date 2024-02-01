/**
 * [408] Valid Word Abbreviation
 *
 * A string can be abbreviated by replacing any number of non-adjacent, non-empty
 * substrings with their lengths. The lengths should not have leading zeros.
 *
 * For example, a string such as "substitution" could be abbreviated as (but not limited to):
 *
 * "s10n" ("s ubstitutio n")
 * "sub4u4" ("sub stit u tion")
 * "12" ("substitution")
 * "su3i1u2on" ("su bst i t u ti on")
 * "substitution" (no substrings replaced)
 * The following are not valid abbreviations:
 *
 * "s55n" ("s ubsti tutio n", the replaced substrings are adjacent)
 * "s010n" (has leading zeros)
 * "s0ubstitution" (replaces an empty substring)
 * Given a string word and an abbreviation abbr, return whether the string matches the given abbreviation.
 *
 * A substring is a contiguous non-empty sequence of characters within a string.
 *
 *
 * Example 1:
 *
 * Input: word = "internationalization", abbr = "i12iz4n"
 * Output: true
 * Explanation: The word "internationalization" can be abbreviated as "i12iz4n" ("i nternational iz atio n").
 * Example 2:
 *
 * Input: word = "apple", abbr = "a2e"
 * Output: false
 * Explanation: The word "apple" cannot be abbreviated as "a2e".
 *
 *
 * Constraints:
 *
 * 1 <= word.length <= 20
 * word consists of only lowercase English letters.
 * 1 <= abbr.length <= 10
 * abbr consists of lowercase English letters and digits.
 * All the integers in abbr will fit in a 32-bit integer.
 *
 * pub fn valid_word_abbreviation(word: String, abbr: String) -> bool {
 *     let (mut number, mut position) = (0 as usize, 0 as usize);
 *     for ch in abbr.chars() {
 *         if ch.is_ascii_digit() {
 *             if ch == '0' && number == 0 { return false; }
 *             number = ch.to_digit(10).unwrap() as usize + number * 10;
 *         } else {
 *             position += number;
 *             number = 0;
 *             if position >= word.len() || word.chars().nth(position).unwrap() != ch {
 *                 return false;
 *             }
 *
 *             position += 1;
 *         }
 *     }
 *
 *     position + number == word.len()
 * }
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/valid-word-abbreviation/
// discuss: https://leetcode.com/problems/valid-word-abbreviation/discuss/?currentPage=1&orderBy=most_votes&query=

impl Solution {
    /// Validates if a given word matches the specified abbreviation.
    ///
    /// The function processes each character in the abbreviation string. If the character
    /// is a digit, it contributes to a cumulative number representing the length of a
    /// substring in the word that should be skipped. If the character is an alphabet,
    /// the function verifies if the word contains the expected character at the current
    /// position after accounting for any skipped characters.
    ///
    /// # Arguments
    ///
    /// * `word` - A `String` representing the word to be matched against the abbreviation.
    /// * `abbr` - A `String` representing the abbreviation to be validated.
    ///
    /// # Returns
    ///
    /// * `true` if the abbreviation accurately represents the word.
    /// * `false` otherwise, such as in cases of leading zeros in numbers or mismatched characters.
    pub fn valid_word_abbreviation(word: String, abbr: String) -> bool {
        // Iterator for the characters in the word, allowing lookahead.
        let mut chars = word.chars().peekable();
        // Accumulator for the current number parsed from the abbreviation.
        let mut num = 0;

        // Iterate over each character in the abbreviation string.
        for c in abbr.chars() {
            if c.is_ascii_digit() {
                // Check for leading zeros: if num is 0 and the digit is '0', the abbreviation is invalid.
                if c == '0' && num == 0 {
                    return false;
                }
                // Parse the digit and update the accumulator.
                num = num * 10 + c.to_digit(10).unwrap() as usize;
            } else {
                // Skip characters in the word according to the accumulated number.
                for _ in 0..num {
                    if chars.next().is_none() {
                        // If the word ends before skipping the required number of characters, return false.
                        return false;
                    }
                }
                // Reset the accumulator when a non-digit is encountered.
                num = 0;
                // Compare the current character of the word with the non-digit character from the abbreviation.
                if Some(c) != chars.next() {
                    // If there's a mismatch, the abbreviation does not match the word.
                    return false;
                }
            }
        }

        // Skip any remaining characters in the word as indicated by the final number in the abbreviation.
        for _ in 0..num {
            if chars.next().is_none() {
                // If the word ends before skipping the required number of characters, return false.
                return false;
            }
        }
        // If there are no more characters left in the word iterator, the abbreviation is valid.
        chars.next().is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_408_true() {
        let word = "internationalization".to_string();
        let abbr = "i12iz4n".to_string();
        assert!(Solution::valid_word_abbreviation(word, abbr));
    }

    #[test]
    fn test_408_false() {
        let word = "apple".to_string();
        let abbr = "a2e".to_string();
        assert!(!Solution::valid_word_abbreviation(word, abbr));
    }

    #[test]
    fn test_408_adjacent_substrings() {
        let word = "substitution".to_string();
        let abbr = "s55n".to_string();
        assert!(!Solution::valid_word_abbreviation(word, abbr));
    }

    #[test]
    fn test_408_leading_zeros() {
        let word = "substitution".to_string();
        let abbr = "s010n".to_string();
        assert!(!Solution::valid_word_abbreviation(word, abbr));
    }

    #[test]
    fn test_408_empty_substring() {
        let word = "substitution".to_string();
        let abbr = "s0ubstitution".to_string();
        assert!(!Solution::valid_word_abbreviation(word, abbr));
    }
}
