/**
 * [691] Stickers to Spell Word
 *
 * We are given n different types of stickers. Each sticker has a lowercase English word on it.
 * You would like to spell out the given string target by cutting individual letters from your collection of stickers and rearranging them. You can use each sticker more than once if you want, and you have infinite quantities of each sticker.
 * Return the minimum number of stickers that you need to spell out target. If the task is impossible, return -1.
 * Note: In all test cases, all words were chosen randomly from the 1000 most common US English words, and target was chosen as a concatenation of two random words.
 *
 * <strong class="example">Example 1:
 *
 * Input: stickers = ["with","example","science"], target = "thehat"
 * Output: 3
 * Explanation:
 * We can use 2 "with" stickers, and 1 "example" sticker.
 * After cutting and rearrange the letters of those stickers, we can form the target "thehat".
 * Also, this is the minimum number of stickers necessary to form the target string.
 *
 * <strong class="example">Example 2:
 *
 * Input: stickers = ["notice","possible"], target = "basicbasic"
 * Output: -1
 * Explanation:
 * We cannot form the target "basicbasic" from cutting letters from the given stickers.
 *
 *
 * Constraints:
 *
 *     n == stickers.length
 *     1 <= n <= 50
 *     1 <= stickers[i].length <= 10
 *     1 <= target.length <= 15
 *     stickers[i] and target consist of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/stickers-to-spell-word/

impl Solution {
    pub fn min_stickers_orig(stickers: Vec<String>, target: String) -> i32 {
        // Length of the target string
        let n = target.len();
        // Calculate the number of possible states (2^n) since each character can be either included or not
        let n_pow = 1 << n;
        // Initialize a dynamic programming table to store the minimum number of stickers needed for each state
        let mut dp = vec![i32::MAX; n_pow];
        // Base case: 0 stickers are needed to form an empty string
        dp[0] = 0;

        // Iterate through all states of the dp table
        for i in 0..n_pow {
            // Only try to use stickers if the current state is reachable
            if dp[i] != i32::MAX {
                Self::try_stickers(&stickers, &target, &mut dp, i);
            }
        }
        // If the final state (all characters included) is not reachable, return -1
        if dp[n_pow - 1] == i32::MAX {
            -1
        } else {
            // Otherwise, return the minimum number of stickers needed for the final state
            dp[n_pow - 1]
        }
    }

    /// Attempts to use each sticker to improve the minimum number of stickers needed for each state.
    /// For the given state `i`, it explores what new states can be reached by applying each sticker.
    fn try_stickers(stickers: &[String], target: &str, dp: &mut [i32], i: usize) {
        // Iterate over each sticker
        for s in stickers {
            // Start with the current state
            let mut now = i;
            // Iterate over each character in the sticker
            for c in s.chars() {
                // Attempt to match the sticker character with the target characters
                for r in 0..target.len() {
                    // Check if the character at position `r` in the target matches the sticker character
                    // and if the character has not been included in the state `now`
                    if target.chars().nth(r).unwrap() == c && (now >> r) & 1 == 0 {
                        // Include the character in the state and break to avoid duplicate usage
                        now |= 1 << r;
                        break;
                    }
                }
            }
            // Update the dp table if the new state `now` can be reached with fewer stickers
            dp[now] = dp[now].min(dp[i] + 1);
        }
    }

    #[allow(clippy::needless_range_loop)]
    pub fn min_stickers(stickers: Vec<String>, target: String) -> i32 {
        // Length of the target string
        let n = target.len();
        // Convert the target string into a byte array for efficient access
        let target = target.as_bytes();
        // Calculate the maximum bitmask representing all characters in the target
        let max_mask = 1 << n;
        // Constant representing the maximum integer value, used as a placeholder for unreachable states
        const MAX_INT: i32 = i32::MAX;
        // Initialize the dynamic programming table with default values
        let mut dp = vec![i32::MAX; max_mask];
        // Base case: 0 stickers are needed to form an empty subset of the target
        dp[0] = 0;

        // Iterate over all possible subsets of characters in the target string
        for mask in 0..max_mask {
            // Skip the current subset if it's unreachable
            if dp[mask] == i32::MAX {
                continue;
            }
            // Iterate over each sticker to try and extend the current subset
            for sticker in stickers.iter() {
                // Temporary variable to hold the new subset that can be created by using the current sticker
                let mut super_mask = mask;
                // Convert the current sticker into a byte array
                let st = (*sticker).as_bytes();
                // Iterate over each character in the sticker
                for j in 0..st.len() {
                    // Compare the sticker character with each character in the target
                    for i in 0..n {
                        // If the characters match and the target character is not yet in the subset
                        if target[i] == st[j] && (super_mask >> i & 1) == 0 {
                            // Add the target character to the subset
                            super_mask |= 1 << i;
                            // Update the dynamic programming table with the new minimum
                            dp[super_mask] = std::cmp::min(dp[super_mask], dp[mask] + 1);
                            // Break to avoid using the same sticker character multiple times
                            break;
                        }
                    }
                }
            }
        }
        // If the final state (all characters included) is unreachable, set it to -1
        if dp[max_mask - 1] == i32::MAX {
            dp[max_mask - 1] = -1;
        }
        // Return the minimum number of stickers needed for the final state
        dp[max_mask - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_691_1() {
        let input = (
            vec![
                "with".to_string(),
                "example".to_string(),
                "science".to_string(),
            ],
            "thehat".to_string(),
        );
        let expected = 3;
        assert_eq!(Solution::min_stickers(input.0, input.1), expected);
    }

    #[test]
    fn test_691_2() {
        let input = (
            vec!["notice".to_string(), "possible".to_string()],
            "basicbasic".to_string(),
        );
        let expected = -1;
        assert_eq!(Solution::min_stickers(input.0, input.1), expected);
    }
}
