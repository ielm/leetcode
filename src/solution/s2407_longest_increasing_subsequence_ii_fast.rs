/**
 * [2407] Longest Increasing Subsequence II
 *
 * You are given an integer array nums and an integer k.
 * Find the longest subsequence of nums that meets the following requirements:
 *
 *     The subsequence is strictly increasing and
 *     The difference between adjacent elements in the subsequence is at most k.
 *
 * Return the length of the longest subsequence that meets the requirements.
 * A subsequence is an array that can be derived from another array by deleting some or no elements without changing the order of the remaining elements.
 *
 * <strong class="example">Example 1:
 *
 * Input: nums = [4,2,1,4,3,4,5,8,15], k = 3
 * Output: 5
 * Explanation:
 * The longest subsequence that meets the requirements is [1,3,4,5,8].
 * The subsequence has a length of 5, so we return 5.
 * Note that the subsequence [1,3,4,5,8,15] does not meet the requirements because 15 - 8 = 7 is larger than 3.
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [7,4,5,1,8,12,4,7], k = 5
 * Output: 4
 * Explanation:
 * The longest subsequence that meets the requirements is [4,5,8,12].
 * The subsequence has a length of 4, so we return 4.
 *
 * <strong class="example">Example 3:
 *
 * Input: nums = [1,5], k = 1
 * Output: 1
 * Explanation:
 * The longest subsequence that meets the requirements is [1].
 * The subsequence has a length of 1, so we return 1.
 *
 *
 * Constraints:
 *
 *     1 <= nums.length <= 10^5
 *     1 <= nums[i], k <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-increasing-subsequence-ii/
// discuss: https://leetcode.com/problems/longest-increasing-subsequence-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Solution Explanation
// This solution uses a segment tree to solve the DP problem.
// Add one to the path length of the longest path found so far among the numbers that are no less than k smaller than the current number. We need a segment tree to be able to to O(log(n)) searches for the max DP entry within an interval

pub struct SegmentTree {
    arr: Vec<i32>,
    len: usize,
}

impl SegmentTree {
    /// Constructs a new, empty `SegmentTree` with the specified length.
    pub fn new(len: usize) -> Self {
        Self {
            arr: vec![0; 2 * len], // Initialize the tree with zeroes
            len,
        }
    }

    /// Constructs a `SegmentTree` from a slice of `i32`.
    pub fn from_slice(src: &[i32]) -> Self {
        let len = src.len();
        let mut arr = vec![0; 2 * len];
        arr[len..].clone_from_slice(src); // Copy the source slice into the second half of the tree
        for i in (1..len).rev() {
            // Populate the first half of the tree with the max values
            arr[i] = arr[i << 1].max(arr[i << 1 | 1]);
        }
        Self { arr, len }
    }

    /// Updates the value at index `i` with the provided `value`, if the `value` is greater
    /// than the current value at that index.
    pub fn update(&mut self, mut i: usize, value: i32) {
        assert!(i < self.len); // Ensure the index is within bounds
        i += self.len; // Adjust the index to the leaf position
        if self.arr[i] >= value {
            // If the current value is already greater or equal, no update is needed
            return;
        }
        self.arr[i] = value; // Set the new value
        while i > 0 {
            // Bubble up the change to the root, updating max values along the way
            self.arr[i >> 1] = self.arr[i].max(self.arr[i ^ 1]);
            i >>= 1;
        }
    }

    /// Queries the maximum value in the range [left, right).
    pub fn query(&mut self, mut left: usize, mut right: usize) -> i32 {
        assert!(left <= right); // Ensure the range is valid
        assert!(right <= self.len); // Ensure the range is within bounds
        let mut res = 0; // Initialize the result
        left += self.len; // Adjust the range to start from the leaves
        right += self.len;
        while left < right {
            if left % 2 == 1 {
                // If left is odd, it's the right child, and its max is taken into account
                res = res.max(self.arr[left]);
                left += 1; // Move to the next range
            }
            if right % 2 == 1 {
                // If right is odd, it's the left child, and its max is taken into account
                right -= 1;
                res = res.max(self.arr[right]);
            }
            // Move up the tree
            left >>= 1;
            right >>= 1;
        }
        res
    }
}

impl Solution {
    /// Finds the length of the longest increasing subsequence with adjacent element difference at most k.
    pub fn length_of_lis(nums: Vec<i32>, k: i32) -> i32 {
        // Initialize the answer to zero
        let mut ans = 0;
        // Create a new SegmentTree with a size sufficient to include all possible values in nums
        let mut tree = SegmentTree::new(100_001);
        for &num in &nums {
            // Calculate the left boundary for the query, ensuring it does not go below 0
            let left = 0.max(num - k) as usize;
            // Query the tree for the maximum length of increasing subsequence ending with a number less than num and within the difference k
            let len = tree.query(left, num as usize);
            // Update the answer with the maximum length found so far
            ans = ans.max(len + 1);
            // Update the tree with the new length of increasing subsequence ending with num
            tree.update(num as usize, len + 1);
        }
        // Return the length of the longest increasing subsequence foun
        ans
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2407_1() {
        let input_arr = vec![4, 2, 1, 4, 3, 4, 5, 8, 15];
        let input_k = 3;
        let expected = 5;
        assert!(Solution::length_of_lis(input_arr, input_k) == expected);
    }

    #[test]
    fn test_2407_2() {
        let input_arr = vec![7, 4, 5, 1, 8, 12, 4, 7];
        let input_k = 5;
        let expected = 4;
        assert!(Solution::length_of_lis(input_arr, input_k) == expected);
    }

    #[test]
    fn test_2407_3() {
        let input_arr = vec![1, 5];
        let input_k = 1;
        let expected = 1;
        assert!(Solution::length_of_lis(input_arr, input_k) == expected);
    }
}
