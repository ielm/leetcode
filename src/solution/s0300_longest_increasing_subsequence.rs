/**
 * [300] Longest Increasing Subsequence
 *
 * Given an integer array nums, return the length of the longest strictly increasing <span data-keyword="subsequence-array">subsequence</span>.
 *
 * <strong class="example">Example 1:
 *
 * Input: nums = [10,9,2,5,3,7,101,18]
 * Output: 4
 * Explanation: The longest increasing subsequence is [2,3,7,101], therefore the length is 4.
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [0,1,0,3,2,3]
 * Output: 4
 *
 * <strong class="example">Example 3:
 *
 * Input: nums = [7,7,7,7,7,7,7]
 * Output: 1
 *
 *
 * Constraints:
 *
 * 1 <= nums.length <= 2500
 * -10^4 <= nums[i] <= 10^4
 *
 *
 * Follow up: Can you come up with an algorithm that runs in O(n log(n)) time complexity?
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-increasing-subsequence/
// discuss: https://leetcode.com/problems/longest-increasing-subsequence/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        // Vector to keep track of the longest increasing subsequence found so far.
        let mut lis = vec![nums[0]];

        // Iterate through each number in the input vector.
        for n in nums.iter() {
            // If the current number is greater than the last number in 'lis', it can extend the subsequence.
            if n > lis.last().unwrap() {
                lis.push(*n);
            } else {
                // If the current number is not greater, find the position to replace in 'lis'.
                // This keeps 'lis' always sorted.
                let pos = lis.binary_search(n);
                // If 'n' is not found, 'binary_search' returns the index where it could be inserted.
                if let Err(pos) = pos {
                    // Replace the element at the found position with 'n' if it is greater than 'n'.
                    // This ensures that we maintain the smallest possible value at this position
                    // to allow for a longer increasing subsequence.
                    if lis[pos] > *n {
                        lis[pos] = *n;
                    }
                }
            }
        }
        // The length of 'lis' is the length of the longest increasing subsequence.
        lis.len() as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_300() {
        let input = vec![10, 9, 2, 5, 3, 7, 101, 18];
        let expected = 4;
        assert_eq!(Solution::length_of_lis(input), expected);

        let input = vec![0, 1, 0, 3, 2, 3];
        let expected = 4;
        assert_eq!(Solution::length_of_lis(input), expected);
    }
}
