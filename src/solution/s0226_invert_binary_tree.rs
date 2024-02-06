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
 *     The number of nodes in the tree is in the range [0, 100].
 *     -100 <= Node.val <= 100
 *
 */
pub struct Solution {}
use crate::util::tree::TreeNode;

// problem: https://leetcode.com/problems/invert-binary-tree/
// discuss: https://leetcode.com/problems/invert-binary-tree/discuss/?currentPage=1&orderBy=most_votes&query=

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

use std::collections::VecDeque;
use std::mem;

trait NodeSwap {
    fn swap(&mut self);
    fn swap_all(&mut self);
}

impl NodeSwap for TreeNode {
    fn swap(&mut self) {
        std::mem::swap(&mut self.left, &mut self.right);
    }

    fn swap_all(&mut self) {
        if let Some(node) = self.left.as_mut() {
            node.borrow_mut().swap_all()
        }
        if let Some(node) = self.right.as_mut() {
            node.borrow_mut().swap_all()
        }
        self.swap();
    }
}

impl Solution {
    pub fn invert_tree_bfs(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        // init a queue
        // add root to queue
        // while queue not empty
        //     pop front of queue
        //     swap l & r nodes of current node
        //     add node children to queue

        let mut queue = VecDeque::new();

        queue.push_back(root.clone());

        while let Some(el) = queue.pop_front() {
            if let Some(n) = el {
                let mut borrowed = n.borrow_mut();
                let borrowed = &mut *borrowed;
                mem::swap(&mut borrowed.right, &mut borrowed.left);
                queue.push_back(borrowed.left.clone());
                queue.push_back(borrowed.right.clone());
            }
        }
        root
    }

    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        root.map(|node| {
            node.borrow_mut().swap_all();
            node
        })
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_226() {}
}
