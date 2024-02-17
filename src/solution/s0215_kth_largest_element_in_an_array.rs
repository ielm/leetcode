use std::cmp::Reverse;
use std::collections::BinaryHeap;
/**
 * [215] Kth Largest Element in an Array
 *
 * Given an integer array nums and an integer k, return the k^th largest element in the array.
 * Note that it is the k^th largest element in the sorted order, not the k^th distinct element.
 * Can you solve it without sorting?
 *  
 * <strong class="example">Example 1:
 * Input: nums = [3,2,1,5,6,4], k = 2
 * Output: 5
 * <strong class="example">Example 2:
 * Input: nums = [3,2,3,1,2,4,5,5,6], k = 4
 * Output: 4
 *  
 * Constraints:
 *
 * 	1 <= k <= nums.length <= 10^5
 * 	-10^4 <= nums[i] <= 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/kth-largest-element-in-an-array/
// discuss: https://leetcode.com/problems/kth-largest-element-in-an-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// heap.push(Reverse(n))

impl Solution {
    pub fn find_kth_largest_binary_heap(nums: Vec<i32>, k: i32) -> i32 {
        let mut heap = BinaryHeap::new();
        for i in nums {
            heap.push(Reverse(i));
            if heap.len() > k as usize {
                heap.pop();
            }
        }
        heap.peek().unwrap().0
    }

    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        // find the kth largest element is equal to find the n-kth smallest element
        *nums.select_nth_unstable(n - k as usize).1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_215_1() {
        let input = vec![3, 2, 1, 5, 6, 4];
        let k = 2;
        let output = 5;
        assert_eq!(Solution::find_kth_largest(input, k), output);
    }

    #[test]
    fn test_215_2() {
        let input = vec![3, 2, 3, 1, 2, 4, 5, 5, 6];
        let k = 4;
        let output = 4;
        assert_eq!(Solution::find_kth_largest(input, k), output);
    }
}
