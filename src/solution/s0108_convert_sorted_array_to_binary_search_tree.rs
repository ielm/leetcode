/**
 * [108] Convert Sorted Array to Binary Search Tree
 *
 * Given an integer array nums where the elements are sorted in ascending order, convert it to a <span data-keyword="height-balanced">height-balanced</span> binary search tree.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/18/btree1.jpg" style="width: 302px; height: 222px;" />
 * Input: nums = [-10,-3,0,5,9]
 * Output: [0,-3,9,-10,null,5]
 * Explanation: [0,-10,5,null,-3,null,9] is also accepted:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/18/btree2.jpg" style="width: 302px; height: 222px;" />
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/18/btree.jpg" style="width: 342px; height: 142px;" />
 * Input: nums = [1,3]
 * Output: [3,1]
 * Explanation: [1,null,3] and [3,1] are both height-balanced BSTs.
 *
 *  
 * Constraints:
 *
 *     1 <= nums.length <= 10^4
 *     -10^4 <= nums[i] <= 10^4
 *     nums is sorted in a strictly increasing order.
 *
 */
pub struct Solution {}
use crate::util::tree::TreeNode;

// problem: https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree/
// discuss: https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn recurse(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() {
            None
        } else {
            let (left, rest) = nums.split_at(nums.len() / 2);
            let (curr, right) = rest.split_first().unwrap();
            Some(Rc::new(RefCell::new(TreeNode {
                val: *curr,
                left: Self::recurse(left),
                right: Self::recurse(right),
            })))
        }
    }

    pub fn sorted_array_to_bst_recursive(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::recurse(&nums)
    }

    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut n = nums.len();

        if n == 0 {
            return None;
        } else {
            let m = n / 2;
            let mut node = TreeNode::new(nums[m]);
            node.left = Self::sorted_array_to_bst(nums[0..m].to_vec());
            node.right = Self::sorted_array_to_bst(nums[m + 1..n].to_vec());

            Some(Rc::new(RefCell::new(node)))
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_108() {}
}
