/**
 * [314] Binary Tree Vertical Order Traversal
 *
 * Given the root of a binary tree, return the vertical order traversal of its nodes' values. (i.e., from top to bottom, column by column).
 * If two nodes are in the same row and column, the order should be from left to right.
 *
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/28/vtree1.jpg" style="width: 282px; height: 301px;" />
 * Input: root = [3,9,20,null,null,15,7]
 * Output: [[9],[3,15],[20],[7]]
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/28/vtree2-1.jpg" style="width: 462px; height: 222px;" />
 * Input: root = [3,9,8,4,0,1,7]
 * Output: [[4],[9],[3,0,1],[8],[7]]
 *
 * <strong class="example">Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/28/vtree2.jpg" style="width: 462px; height: 302px;" />
 * Input: root = [3,9,8,4,0,1,7,null,null,null,2,5]
 * Output: [[4],[9,5],[3,0,1],[8,2],[7]]
 *
 *
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [0, 100].
 * 	-100 <= Node.val <= 100
 *
 */
pub struct Solution {}
// use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/binary-tree-vertical-order-traversal/
// discuss: https://leetcode.com/problems/binary-tree-vertical-order-traversal/discuss/?currentPage=1&orderBy=most_votes&query=

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
use std::collections::{BTreeMap, HashMap, VecDeque};
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

impl Solution {
    pub fn vertical_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return vec![];
        }

        let mut queue = VecDeque::from([(0, root)]);
        // let mut col_map = HashMap::<i32, Vec<i32>>::new();
        let mut col_map = BTreeMap::<i32, Vec<i32>>::new();

        // Run the BFS
        while let Some((col, node)) = queue.pop_front() {
            // If node is some, populate the column table with the nodes col index and the node
            if let Some(rcn) = node {
                // Populate the column table with node and col index
                col_map.entry(col).or_default().push(rcn.borrow().val);

                // add child nodes to the queue
                queue.push_back((col - 1, rcn.borrow_mut().left.take()));
                queue.push_back((col + 1, rcn.borrow_mut().right.take()));
            }
        }

        // Return the column table as a vector
        col_map.into_values().collect()

    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_314() {}
}
