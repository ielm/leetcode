/**
 * [226] Invert Binary Tree
 *
 * Given the root of a binary tree, invert the tree, and return its root.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/14/invert1-tree.jpg" style="width: 500px; height: 165px;" />
 * Input: root = [4,2,7,1,3,6,9]
 * Output: [4,7,2,9,6,3,1]
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/14/invert2-tree.jpg" style="width: 500px; height: 120px;" />
 * Input: root = [2,1,3]
 * Output: [2,3,1]
 *
 * <strong class="example">Example 3:
 *
 * Input: root = []
 * Output: []
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [0, 100].
 * 	-100 <= Node.val <= 100
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/invert-binary-tree/
// discuss: https://leetcode.com/problems/invert-binary-tree/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::borrow::Borrow;
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
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        // init a queue
        // add root to queue
        // while queue not empty
        //     pop front of queue
        //     swap l & r nodes of current node
        //     add node children to queue

        let queue = VecDeque::new();
        queue.push_back(root);

        while let Some(node) = queue.pop_front() {
            let temp = root.borrow().left;
            root.left = root.borrow().right;
            root.right = temp;

            if let Some(lc) = root.left {
                queue.push_back(lc);
            }
            if let Some(rc) = root.right {
                queue.push_back(rc)
            }
        }
        root
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_226() {}
}
