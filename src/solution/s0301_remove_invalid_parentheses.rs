/**
 * [301] Remove Invalid Parentheses
 *
 * Given a string s that contains parentheses and letters, remove the minimum number of invalid parentheses to make the input string valid.
 * Return a list of unique strings that are valid with the minimum number of removals. You may return the answer in any order.
 *
 * <strong class="example">Example 1:
 *
 * Input: s = "()())()"
 * Output: ["(())()","()()()"]
 *
 * <strong class="example">Example 2:
 *
 * Input: s = "(a)())()"
 * Output: ["(a())()","(a)()()"]
 *
 * <strong class="example">Example 3:
 *
 * Input: s = ")("
 * Output: [""]
 *
 *
 * Constraints:
 *
 *     1 <= s.length <= 25
 *     s consists of lowercase English letters and parentheses '(' and ')'.
 *     There will be at most 20 parentheses in s.
 *
 */
use std::char;

pub struct Solution {}

// problem: https://leetcode.com/problems/remove-invalid-parentheses/
// discuss: https://leetcode.com/problems/remove-invalid-parentheses/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
        let mut ans: Vec<String> = Vec::new();
        // Start the recursive removal process with the initial string,
        // an empty answer vector, and the default pair of parentheses to check.
        Self::remove(&s, &mut ans, 0, 0, &['(', ')']);
        ans
    }

    fn remove(s: &str, ans: &mut Vec<String>, last_i: usize, last_j: usize, par: &[char; 2]) {
        let mut stack = 0; // Counter to track the balance of parentheses
        let mut i = last_i; // Position to start scanning the string

        // Scan the string from the last position to check the balance of parentheses
        while i < s.len() {
            if s.chars().nth(i).unwrap() == par[0] {
                stack += 1; // Found an opening parenthesis, increase stack
            }
            if s.chars().nth(i).unwrap() == par[1] {
                stack -= 1; // Found a closing parenthesis, decrease stack
            }
            // If the stack is balanced or has more opening parentheses, continue scanning
            if stack >= 0 {
                i += 1;
                continue;
            }
            // Found an imbalance, need to remove a closing parenthesis
            for j in last_j..=i {
                // Only consider removal if it's a closing parenthesis and either the first one
                // or not preceded by another closing parenthesis (to avoid duplicates)
                if s.chars().nth(j).unwrap() == par[1]
                    && (j == last_j || s.chars().nth(j - 1).unwrap() != par[1])
                {
                    // Create a new string without the j-th character
                    let mut new_s = s[0..j].to_string();
                    new_s.push_str(&s[j + 1..]);
                    // Continue the removal process with the new string
                    Solution::remove(&new_s, ans, i, j, par);
                }
            }
            return; // Return early as we've made a removal and need to check the new string
        }
        // After scanning the whole string, reverse it if we were checking for '('
        let reversed: String = s.chars().rev().collect();
        if par[0] == '(' {
            // If we were checking for opening parentheses, now check for closing ones
            Solution::remove(&reversed, ans, 0, 0, &[')', '(']);
        } else {
            // If we were checking for closing parentheses, we're done and can add the result
            ans.push(reversed);
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_301_1() {
        assert_eq!(
            Solution::remove_invalid_parentheses("()())()".to_string()),
            vec!["(())()".to_string(), "()()()".to_string()]
        );
    }

    #[test]
    fn test_301_2() {
        assert_eq!(
            Solution::remove_invalid_parentheses("(a)())()".to_string()),
            vec!["(a())()".to_string(), "(a)()()".to_string()]
        );
    }

    #[test]
    fn test_301_3() {
        assert_eq!(
            Solution::remove_invalid_parentheses(")(".to_string()),
            vec!["".to_string()]
        );
    }
}
