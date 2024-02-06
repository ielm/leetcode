/**
 * [49] Group Anagrams
 *
 * Given an array of strings strs, group the anagrams together. You can return the answer in any order.
 * An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.
 *
 * <strong class="example">Example 1:
 * Input: strs = ["eat","tea","tan","ate","nat","bat"]
 * Output: [["bat"],["nat","tan"],["ate","eat","tea"]]
 * <strong class="example">Example 2:
 * Input: strs = [""]
 * Output: [[""]]
 * <strong class="example">Example 3:
 * Input: strs = ["a"]
 * Output: [["a"]]
 *
 * Constraints:
 *
 *     1 <= strs.length <= 10^4
 *     0 <= strs[i].length <= 100
 *     strs[i] consists of lowercase English letters.
 *
 */
use std::collections::HashMap;

pub struct Solution {}

// problem: https://leetcode.com/problems/group-anagrams/
// discuss: https://leetcode.com/problems/group-anagrams/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        for s in strs {
            let mut counter = [0; 26];
            for c in s.chars() {
                counter[(c as usize) - ('a' as usize)] += 1;
            }
            let sorted_str = counter
                .iter()
                .enumerate()
                .flat_map(|(i, &count)| std::iter::repeat((i as u8 + b'a') as char).take(count))
                .collect::<String>();
            map.entry(sorted_str).or_default().push(s);
        }
        map.into_values().collect()
    }

    fn sorted_map_group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        // Create a HashMap to store the anagrams.
        // The key is the sorted string, and the value is a vector of anagrams.
        // For example, "eat" and "tea" will be stored under the key "aet".
        let mut map = std::collections::HashMap::new();

        // Iterate through each string in the input vector.
        for s in strs {
            // Sort the string and store it in the HashMap.
            // If the sorted string is not in the HashMap, create a new vector.
            // Then, push the original string into the vector.
            let slice = &s[..];
            let mut chars: Vec<char> = slice.chars().collect();
            chars.sort();

            map.entry(chars).or_insert(vec![]).push(s);
        }

        // Return the values of the HashMap as a vector of vectors.
        map.into_values().collect()
    }

    // Since the string contains only lower case characters, we can sort them usin gcounting sort
    // which will reduce the time complexity to O(n)
    fn counted_group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<[u8; 26], Vec<String>> = HashMap::new();
        for s in strs {
            let mut count = [0; 26];
            for &c in s.as_bytes() {
                count[(c - b'a') as usize] += 1;
            }
            map.entry(count).or_default().push(s);
        }
        map.into_values().collect()
    }

    fn unsafe_group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        for s in strs {
            let mut sorted = s.clone();
            unsafe {
                sorted.as_mut_vec().sort();
            }
            map.entry(sorted)
                .and_modify(|x| x.push(s.clone()))
                .or_insert(vec![s]);
        }
        map.into_values().collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_49_1() {
        let input = vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ];
        let mut result = Solution::group_anagrams(input);
        for group in &mut result {
            group.sort();
        }
        result.sort();
        let mut expected = vec![
            vec!["bat".to_string()],
            vec!["nat".to_string(), "tan".to_string()],
            vec!["ate".to_string(), "eat".to_string(), "tea".to_string()],
        ];
        for group in &mut expected {
            group.sort();
        }
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_49_2() {
        let input = vec!["".to_string()];
        let mut result = Solution::group_anagrams(input);
        for group in &mut result {
            group.sort();
        }
        result.sort();
        let mut expected = vec![vec!["".to_string()]];
        for group in &mut expected {
            group.sort();
        }
        expected.sort();
        assert_eq!(result, expected);
    }
}
