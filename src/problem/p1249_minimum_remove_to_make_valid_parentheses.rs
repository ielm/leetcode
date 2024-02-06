use std::{char, collections::HashSet};

/**
 * [1249] Minimum Remove to Make Valid Parentheses
 *
 * Given a string <font face="monospace">s</font> of '(' , ')' and lowercase English characters.
 * Your task is to remove the minimum number of parentheses ( '(' or ')', in any positions ) so that the resulting parentheses string is valid and return any valid string.
 * Formally, a parentheses string is valid if and only if:
 *
 *     It is the empty string, contains only lowercase characters, or
 *     It can be written as AB (A concatenated with B), where A and B are valid strings, or
 *     It can be written as (A), where A is a valid string.
 *
 *
 * <strong class="example">Example 1:
 *
 * Input: s = "lee(t(c)o)de)"
 * Output: "lee(t(c)o)de"
 * Explanation: "lee(t(co)de)" , "lee(t(c)ode)" would also be accepted.
 *
 * <strong class="example">Example 2:
 *
 * Input: s = "a)b(c)d"
 * Output: "ab(c)d"
 *
 * <strong class="example">Example 3:
 *
 * Input: s = "))(("
 * Output: ""
 * Explanation: An empty string is also valid.
 *
 *
 * Constraints:
 *
 *     1 <= s.length <= 10^5
 *     s[i] is either'(' , ')', or lowercase English letter.
 *
 */

/// Removes the minimum number of parentheses to make the input string valid.
///
/// The function ensures that the resulting string adheres to the rules for a valid parentheses string,
/// which are:
/// - It is an empty string.
/// - It contains only lowercase characters.
/// - It is a concatenation of two valid strings.
/// - It is a single valid string wrapped in parentheses.
///
/// The approach involves scanning the string and using a stack to keep track of the positions
/// of open parentheses '('. When a closing parenthesis ')' is encountered, the function attempts
/// to pop from the stack. If the stack is empty, it means there is no matching opening parenthesis,
/// and the index of the closing parenthesis is marked for removal. After processing the entire
/// string, any remaining indices in the stack (unmatched opening parentheses) are also marked for
/// removal. Finally, the function reconstructs the string, omitting characters at the marked indices.
///
/// # Arguments
///
/// * `s` - A string slice containing the input string with parentheses and lowercase letters.
///
/// # Returns
///
/// A `String` that is a valid parentheses string after the removal of unmatched parentheses.
fn solution(s: &str) -> String {
    // Set to hold the indices of parentheses to be removed.
    let mut indexes_to_remove = HashSet::new();
    // Stack to keep track of the indices of opening parentheses.
    let mut stack = Vec::new();

    // Iterate over the characters of the string, along with their indices.
    for (i, c) in s.char_indices() {
        match c {
            // If an opening parenthesis is found, push its index onto the stack.
            '(' => stack.push(i),
            // If a closing parenthesis is found, attempt to pop from the stack.
            ')' => {
                if stack.pop().is_some() {
                    // If pop is successful, we had a matching opening parenthesis; continue.
                    continue;
                }
                // If the stack is empty, mark this closing parenthesis for removal.
                indexes_to_remove.insert(i);
            }
            // For any other character, do nothing.
            _ => {}
        }
    }

    // Any remaining indices in the stack represent unmatched opening parentheses.
    // Add them to the set of indices to remove.
    for &open_index in &stack {
        indexes_to_remove.insert(open_index);
    }

    // Reconstruct the string, filtering out characters at the marked indices.
    s.char_indices()
        .filter(|(i, _)| !indexes_to_remove.contains(i))
        .map(|(_, c)| c)
        .collect()
}
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-remove-to-make-valid-parentheses/
// discuss: https://leetcode.com/problems/minimum-remove-to-make-valid-parentheses/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    /// Removes the minimum number of parentheses to make the input string valid.
    ///
    /// The function ensures that the resulting string adheres to the rules for a valid parentheses string,
    /// which are:
    /// - It is an empty string.
    /// - It contains only lowercase characters.
    /// - It is a concatenation of two valid strings.
    /// - It is a single valid string wrapped in parentheses.
    ///
    /// The approach involves scanning the string and using a stack to keep track of the positions
    /// of open parentheses '('. When a closing parenthesis ')' is encountered, the function attempts
    /// to pop from the stack. If the stack is empty, it means there is no matching opening parenthesis,
    /// and the index of the closing parenthesis is marked for removal. After processing the entire
    /// string, any remaining indices in the stack (unmatched opening parentheses) are also marked for
    /// removal. Finally, the function reconstructs the string, omitting characters at the marked indices.
    ///
    /// # Arguments
    ///
    /// * `s` - A string slice containing the input string with parentheses and lowercase letters.
    ///
    /// # Returns
    ///
    /// A `String` that is a valid parentheses string after the removal of unmatched parentheses.
    fn original_solution(s: &str) -> String {
        // Set to hold the indices of parentheses to be removed.
        let mut indexes_to_remove = HashSet::new();
        // Stack to keep track of the indices of opening parentheses.
        let mut stack = Vec::new();

        // Iterate over the characters of the string, along with their indices.
        for (i, c) in s.char_indices() {
            match c {
                // If an opening parenthesis is found, push its index onto the stack.
                '(' => stack.push(i),
                // If a closing parenthesis is found, attempt to pop from the stack.
                ')' => {
                    if stack.pop().is_some() {
                        // If pop is successful, we had a matching opening parenthesis; continue.
                        continue;
                    }
                    // If the stack is empty, mark this closing parenthesis for removal.
                    indexes_to_remove.insert(i);
                }
                // For any other character, do nothing.
                _ => {}
            }
        }

        // Any remaining indices in the stack represent unmatched opening parentheses.
        // Add them to the set of indices to remove.
        for &open_index in &stack {
            indexes_to_remove.insert(open_index);
        }

        // Reconstruct the string, filtering out characters at the marked indices.
        s.char_indices()
            .filter(|(i, _)| !indexes_to_remove.contains(i))
            .map(|(_, c)| c)
            .collect()
    }

    /// Removes the minimum number of invalid parentheses from the input string.
    ///
    /// This function first removes invalid closing parentheses by using a stack to track
    /// the indices of opening parentheses. It then reverses the string and removes invalid
    /// opening parentheses using the same logic. The result is a valid parentheses string.
    ///
    /// # Arguments
    ///
    /// * `s` - A `String` containing the input string with parentheses and lowercase letters.
    ///
    /// # Returns
    ///
    /// A `String` that is a valid parentheses string after the removal of unmatched parentheses.
    pub fn min_remove_to_make_valid(s: String) -> String {
        // First pass: remove invalid closing parentheses.
        let s = Self::string_builder(s, '(', ')');
        // Second pass: remove invalid opening parentheses by reversing the string
        // and treating ')' as '(' and '(' as ')'.
        Self::string_builder(s.chars().rev().collect(), ')', '(')
    }

    /// Helper function to remove invalid parentheses in one direction (left-to-right or right-to-left).
    ///
    /// It iterates over the characters of the string, tracking the balance of parentheses.
    /// If it finds an excess closing parenthesis without a matching opening one, it skips adding it.
    /// For opening parentheses, it increments the balance and adds the character.
    /// For closing parentheses, it decrements the balance (if positive) and adds the character.
    /// Other characters are added unconditionally.
    ///
    /// # Arguments
    ///
    /// * `s` - A `String` containing the input string to process.
    /// * `open` - A `char` representing the opening parenthesis character.
    /// * `close` - A `char` representing the closing parenthesis character.
    ///
    /// # Returns
    ///
    /// A `String` that has removed invalid parentheses in the specified direction.
    pub fn string_builder(s: String, open: char, close: char) -> String {
        let mut out = String::new(); // Output string with valid parentheses.
        let mut balance = 0; // Balance of open-to-close parentheses.

        // Iterate over the characters of the string.
        for c in s.chars() {
            if c == open {
                // Increment balance for an opening parenthesis and add it to the output.
                balance += 1;
                out.push(c);
            } else if c == close {
                // Skip the closing parenthesis if there is no matching opening one.
                if balance == 0 {
                    continue;
                }
                // Decrement balance for a closing parenthesis and add it to the output.
                balance -= 1;
                out.push(c);
            } else {
                // Add any non-parenthesis character to the output.
                out.push(c);
            }
        }

        out // Return the constructed string with valid parentheses.
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_1249() {}
}

// let mut removal_idxs = HashSet::new();

// let mut stack = vec![];

// for (i, c) in s.char_indices() {
//     match c {
//         '(' => stack.push(i),
//         ')' => {
//             if stack.pop().is_some() {
//                 continue;
//             }
//             removal_idxs.insert(i);
//         }
//         _ => {}
//     }
// }

// removal_idxs.extend(stack);

// s.char_indices()
//     .filter(|(i, _)| !removal_idxs.contains(i))
//     .map(|(_, c)| c)
//     .collect()
