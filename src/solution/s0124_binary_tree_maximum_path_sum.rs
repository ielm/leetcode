/**
 * [124] Binary Tree Maximum Path Sum
 *
 * A path in a binary tree is a sequence of nodes where each pair of adjacent nodes in the sequence has an edge connecting them. A node can only appear in the sequence at most once. Note that the path does not need to pass through the root.
 * The path sum of a path is the sum of the node's values in the path.
 * Given the root of a binary tree, return the maximum path sum of any non-empty path.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/13/exx1.jpg" style="width: 322px; height: 182px;" />
 * Input: root = [1,2,3]
 * Output: 6
 * Explanation: The optimal path is 2 -> 1 -> 3 with a path sum of 2 + 1 + 3 = 6.
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/13/exx2.jpg" />
 * Input: root = [-10,9,20,null,null,15,7]
 * Output: 42
 * Explanation: The optimal path is 15 -> 20 -> 7 with a path sum of 15 + 20 + 7 = 42.
 *
 *  
 * Constraints:
 *
 *     The number of nodes in the tree is in the range [1, 3 * 10^4].
 *     -1000 <= Node.val <= 1000
 *
 */
pub struct Solution {}
use crate::util::tree::TreeNode;

// problem: https://leetcode.com/problems/binary-tree-maximum-path-sum/
// discuss: https://leetcode.com/problems/binary-tree-maximum-path-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_sum = i32::MIN;
        Self::gain_from_subtree(&root, &mut max_sum);
        std::cmp::max(max_sum, 0)
    }

    fn gain_from_subtree(root: &Option<Rc<RefCell<TreeNode>>>, max_sum: &mut i32) -> i32 {
        if let Some(node) = root {
            let val = node.borrow().val;
            let l = std::cmp::max(Self::gain_from_subtree(&node.borrow().left, max_sum), 0);
            let r = std::cmp::max(Self::gain_from_subtree(&node.borrow().right, max_sum), 0);
            *max_sum = std::cmp::max(*max_sum, val + l + r);
            val + std::cmp::max(l, r)
        } else {
            0
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::tree::to_tree;

    #[test]
    fn test_124_1() {
        assert_eq!(Solution::max_path_sum(tree![1, 2, 3]), 6);
    }

    #[test]
    fn test_124_2() {
        assert_eq!(
            Solution::max_path_sum(tree![-10, 9, 20, null, null, 15, 7]),
            42
        );
    }

    #[test]
    fn test_124_3() {
        assert_eq!(Solution::max_path_sum(tree![1, 2]), 3);
    }

    #[test]
    fn test_124_4() {
        assert_eq!(Solution::max_path_sum(tree![]), 0);
    }
}
