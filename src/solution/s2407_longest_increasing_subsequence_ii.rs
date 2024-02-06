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

pub struct SegmentTree<T> {
    n: usize,     // The size of the input array
    tree: Vec<T>, // The segment tree represented as a vector
}

impl<T> SegmentTree<T>
where
    T: Copy + Ord + Default, // T must support copy, ordering, and have a default value
{
    // Constructs a new Segment Tree from a slice of elements
    pub fn new(nums: &[T]) -> Self {
        let n = nums.len();
        // Initialize the tree with default values and then the input slice
        let mut tree: Vec<_> = std::iter::repeat(T::default())
            .take(n)
            .chain(nums.iter().copied())
            .collect();
        // Build the tree by calculating parents as the max of their children
        for i in (1..n).rev() {
            tree[i] = tree[2 * i].max(tree[2 * i + 1]);
        }
        Self { n, tree }
    }

    // Updates the value at a specific index and propagates the change up the tree
    pub fn update(&mut self, index: usize, val: T) {
        let mut i = index + self.n; // Index in the tree
        self.tree[i] = val; // Set the value at the leaf
                            // Propagate changes up the tree
        while i > 0 {
            let (child1, child2) = (i, if i % 2 == 0 { i + 1 } else { i - 1 });
            i /= 2;
            // Parent takes the max of its children
            self.tree[i] = self.tree[child1].max(self.tree[child2]);
        }
    }

    // Returns the maximum value within the range [left, right]
    pub fn max_range(&self, left: usize, right: usize) -> T {
        let (mut left, mut right) = (left + self.n, right + self.n); // Adjusted indices
        let mut max = T::default(); // Start with the default value

        while left <= right {
            if left % 2 == 1 {
                max = max.max(self.tree[left]); // If left is a right child, take its value
                left += 1; // Move to the next value
            }
            if right % 2 == 0 {
                max = max.max(self.tree[right]); // If right is a left child, take its value
                right -= 1; // Move to the previous value
            }
            left /= 2; // Move up the tree
            right /= 2; // Move up the tree
        }

        max // Return the maximum value found
    }
}

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>, k: i32) -> i32 {
        // Initialize the dynamic programming table using a segment tree.
        // The segment tree is initialized with a size of 100002, which is
        // slightly larger than the maximum possible value of nums[i] and k.
        let mut dp = SegmentTree::new(&[0; 100002]);
        // This variable will keep track of the length of the longest increasing subsequence found so far.
        let mut global_max = 1;

        // Iterate over each number in the input array to build the DP table.
        for num in nums {
            // Calculate the range within which we can find the previous elements of the subsequence.
            // We ensure that the low end of the range is not negative by using 0.max(num - k).
            // Since the subsequence is strictly increasing, we look for the previous number up to num - 1.
            let (low, high) = (0.max(num - k) as usize, 0.max(num - 1) as usize);
            // Query the segment tree for the maximum length of the subsequence within the range [low, high].
            // We add 1 to the result because we are including the current number in the subsequence.
            let max = dp.max_range(low, high) + 1;
            // Update the global maximum length if the current subsequence is longer.
            global_max = global_max.max(max);
            // Update the DP table with the new maximum length for the current number.
            dp.update(num as usize, max);
        }

        // After processing all numbers, return the length of the longest increasing subsequence.
        global_max
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
