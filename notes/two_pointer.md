# Two Pointer Technique

The two pointer technique is a strategy that involves using two pointers (or cursors) to solve a problem. The pointers can be used to iterate over a sequence, search for a target, or solve a problem with a specific constraint. The technique is often used to solve problems that involve arrays, strings, or linked lists.

The two pointer technique is a generalization of the sliding window technique, which is a specific application of the two pointer technique. The sliding window technique is used to solve problems that involve finding a subarray or substring that satisfies a specific condition. The technique is called "sliding window" because the window (or subarray) is moved from left to right, one element at a time.

Sample Rust Solution for [Valid Word Abbreviation](https://leetcode.com/problems/valid-word-abbreviation):

```rust
/// Checks if a given abbreviation is valid for a specific word.
///
/// The function iterates over each character of the abbreviation. If the character is a digit,
/// it is added to the current number being formed. If the character is not a digit, the function
/// checks if the accumulated number of characters (if any) can be skipped in the word and if the
/// next character matches the non-digit character from the abbreviation.
///
/// # Arguments
///
/// * `word` - A string slice representing the word to be checked.
/// * `abbr` - A string slice representing the abbreviation to be validated.
///
/// # Returns
///
/// A boolean indicating whether the abbreviation is valid for the word.
pub fn valid_word_abbreviation(word: &str, abbr: &str) -> bool {
    let mut number = 0; // Accumulated number of characters to skip
    let mut position = 0; // Current position in the word

    for ch in abbr.chars() {
        // Accumulate the number if the character is a digit
        // Otherwise, check if the accumulated number of characters can be skipped
        if ch.is_ascii_digit() {
            // Leading zeros are not allowed
            if ch == '0' && number == 0 {
                return false;
            }
            // Update the accumulated number
            number = number * 10 + ch.to_digit(10).unwrap() as usize;
        } else {
            // Move the position forward by the accumulated number
            position += number;
            // Reset the accumulated number
            number = 0;
            // Check if the position is within bounds and characters match
            if position >= word.len() || word.chars().nth(position).unwrap() != ch {
                return false;
            }
            // Move to the next character
            position += 1;
        }
    }

    // The word is valid if the final position plus skipped characters equals the word's length
    position + number == word.len()
}
```
