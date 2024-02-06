/**
 * [387] First Unique Character in a String
 *
 * Given a string s, find the first non-repeating character in it and return its index. If it does not exist, return -1.
 *
 * <strong class="example">Example 1:
 * Input: s = "leetcode"
 * Output: 0
 * <strong class="example">Example 2:
 * Input: s = "loveleetcode"
 * Output: 2
 * <strong class="example">Example 3:
 * Input: s = "aabb"
 * Output: -1
 *
 * Constraints:
 *
 *     1 <= s.length <= 10^5
 *     s consists of only lowercase English letters.
 *
 */

pub struct Solution {}

// problem: https://leetcode.com/problems/first-unique-character-in-a-string/
// discuss: https://leetcode.com/problems/first-unique-character-in-a-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        // Create an array to keep track of the frequency of each character in the string.
        // Since the input string consists of only lowercase English letters,
        // an array of 26 elements is sufficient to cover all the characters ('a' to 'z').
        let mut seen = [0; 26];

        // Iterate over each byte in the string, incrementing the frequency count
        // of the corresponding character in the 'seen' array.
        for ch in s.bytes() {
            seen[(ch - b'a') as usize] += 1;
        }

        // Iterate over each byte in the string again, this time checking the frequency count
        // of each character. If the frequency count is 1, it means the character is unique
        // and has not been repeated in the string. Return the index of the first such character.
        for (i, ch) in s.bytes().enumerate() {
            if seen[(ch - b'a') as usize] == 1 {
                return i as i32;
            }
        }

        // If no unique character is found, return -1.
        -1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_387_1() {
        let input = "leetcode".to_string();
        let output = 0;
        assert_eq!(Solution::first_uniq_char(input), output);
    }

    #[test]
    fn test_387_2() {
        let input = "loveleetcode".to_string();
        let output = 2;
        assert_eq!(Solution::first_uniq_char(input), output);
    }

    #[test]
    fn test_387_3() {
        let input = "aabb".to_string();
        let output = -1;
        assert_eq!(Solution::first_uniq_char(input), output);
    }
}
