/**
 * [242] Valid Anagram
 *
 * Given two strings s and t, return true if t is an anagram of s, and false otherwise.
 * An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.
 *
 * <strong class="example">Example 1:
 * Input: s = "anagram", t = "nagaram"
 * Output: true
 * <strong class="example">Example 2:
 * Input: s = "rat", t = "car"
 * Output: false
 *
 * Constraints:
 *
 * 	1 <= s.length, t.length <= 5 * 10^4
 * 	s and t consist of lowercase English letters.
 *
 *
 * Follow up: What if the inputs contain Unicode characters? How would you adapt your solution to such a case?
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/valid-anagram/
// discuss: https://leetcode.com/problems/valid-anagram/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        // Create a HashMap to store the frequency of each character in the string `s`.
        let mut map = std::collections::HashMap::new();

        // Increment the count for each character in string `s`.
        s.chars().for_each(|c| *map.entry(c).or_insert(0) += 1);

        // Decrement the count for each character in string `t`.
        t.chars().for_each(|c| *map.entry(c).or_insert(0) -= 1);

        // Check if all counts in the HashMap are zero, indicating an anagram.
        // If any count is not zero, the strings are not anagrams.
        !map.into_values().any(|count| count != 0)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_242_1() {
        let input = ("anagram".to_string(), "nagaram".to_string());
        let expected = true;
        assert_eq!(Solution::is_anagram(input.0, input.1), expected);
    }

    #[test]
    fn test_242_2() {
        let input = ("rat".to_string(), "car".to_string());
        let expected = false;
        assert_eq!(Solution::is_anagram(input.0, input.1), expected);
    }
}
