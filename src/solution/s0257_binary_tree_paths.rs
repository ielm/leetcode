/**
 * [257] Binary Tree Paths
 *
 * Given the root of a binary tree, return all root-to-leaf paths in any order.
 * A leaf is a node with no children.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/12/paths-tree.jpg" style="width: 207px; height: 293px;" />
 * Input: root = [1,2,3,null,5]
 * Output: ["1->2->5","1->3"]
 *
 * <strong class="example">Example 2:
 *
 * Input: root = [1]
 * Output: ["1"]
 *
 *  
 * Constraints:
 *
 *     The number of nodes in the tree is in the range [1, 100].
 *     -100 <= Node.val <= 100
 *
 */
pub struct Solution {}
use crate::util::tree::TreeNode;

// problem: https://leetcode.com/problems/binary-tree-paths/
// discuss: https://leetcode.com/problems/binary-tree-paths/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut ans = Vec::new(); // Initialize the answer to store the paths.

        if let Some(root) = root {
            Self::solve(root, "".to_string(), &mut ans);
        }

        ans
    }

    fn solve(node: Rc<RefCell<TreeNode>>, path: String, ans: &mut Vec<String>) {
        if node.borrow().left.is_none() && node.borrow().right.is_none() {
            ans.push(format!("{}{}", path, node.borrow().val));
        }
        if node.borrow().left.is_some() {
            Self::solve(
                node.borrow().left.as_ref().unwrap().clone(),
                format!("{}{}->", path, node.borrow().val),
                ans,
            );
        }
        if node.borrow().right.is_some() {
            Self::solve(
                node.borrow().right.as_ref().unwrap().clone(),
                format!("{}{}->", path, node.borrow().val),
                ans,
            );
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::tree::to_tree;

    #[test]
    fn test_257_1() {
        let input = to_tree(vec![Some(1), Some(2), Some(3), None, Some(5)]);
        let result = vec!["1->2->5".to_string(), "1->3".to_string()];
        assert_eq!(Solution::binary_tree_paths(input), result);
    }

    #[test]
    fn test_257_2() {
        let input = to_tree(vec![Some(1)]);

        let result = vec!["1".to_string()];
        assert_eq!(Solution::binary_tree_paths(input), result);
    }

    #[test]
    fn test_257_3() {
        let input = to_tree(vec![Some(1), Some(2)]);
        let result = vec!["1->2".to_string()];
        assert_eq!(Solution::binary_tree_paths(input), result);
    }
}
