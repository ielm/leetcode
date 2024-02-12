/**
 * [236] Lowest Common Ancestor of a Binary Tree
 *
 * Given a binary tree, find the lowest common ancestor (LCA) of two given nodes in the tree.
 * According to the <a href="https://en.wikipedia.org/wiki/Lowest_common_ancestor" target="_blank">definition of LCA on Wikipedia</a>: &ldquo;The lowest common ancestor is defined between two nodes p and q as the lowest node in T that has both p and q as descendants (where we allow a node to be a descendant of itself).&rdquo;
 *
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2018/12/14/binarytree.png" style="width: 200px; height: 190px;" />
 * Input: root = [3,5,1,6,2,0,8,null,null,7,4], p = 5, q = 1
 * Output: 3
 * Explanation: The LCA of nodes 5 and 1 is 3.
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2018/12/14/binarytree.png" style="width: 200px; height: 190px;" />
 * Input: root = [3,5,1,6,2,0,8,null,null,7,4], p = 5, q = 4
 * Output: 5
 * Explanation: The LCA of nodes 5 and 4 is 5, since a node can be a descendant of itself according to the LCA definition.
 *
 * <strong class="example">Example 3:
 *
 * Input: root = [1,2], p = 1, q = 2
 * Output: 1
 *
 *
 * Constraints:
 *
 *     The number of nodes in the tree is in the range [2, 10^5].
 *     -10^9 <= Node.val <= 10^9
 *     All Node.val are unique.
 *     p != q
 *     p and q will exist in the tree.
 *
 */
use crate::util::tree::TreeNode;

// problem: https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/
// discuss: https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::cell::RefCell;
use std::rc::Rc;
pub struct Solution {}

impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match (root, p, q) {
            (Some(node), Some(p), Some(q)) => {
                if node.borrow().val == p.borrow().val || node.borrow().val == q.borrow().val {
                    return Some(node);
                }

                let left = Self::lowest_common_ancestor(
                    node.borrow().left.clone(),
                    Some(Rc::clone(&p)),
                    Some(Rc::clone(&q)),
                );

                let right = Self::lowest_common_ancestor(
                    node.borrow().right.clone(),
                    Some(Rc::clone(&p)),
                    Some(Rc::clone(&q)),
                );

                if left.is_some() && right.is_some() {
                    Some(node)
                } else {
                    left.or(right)
                }
            }
            _ => None,
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::tree::to_tree;

    //        3
    //       / \
    //      5   1
    //     / \ / \
    //    6  2 0  8
    //      / \
    //     7   4
    #[test]
    fn test_236_1() {
        let root = to_tree(vec![
            Some(3),
            Some(5),
            Some(1),
            Some(6),
            Some(2),
            Some(0),
            Some(8),
            None,
            None,
            Some(7),
            Some(4),
        ]);
        let p = to_tree(vec![Some(5), Some(6), Some(2), Some(7), Some(4)]);
        let q = to_tree(vec![Some(1), Some(0), Some(8)]);
        assert_eq!(
            Solution::lowest_common_ancestor(root.clone(), p, q),
            root.clone()
        );
    }

    //        3
    //       / \
    //      5   1
    //     / \ / \
    //    6  2 0  8
    //      / \
    //     7   4
    #[test]
    fn test_236_2() {
        let root = to_tree(vec![
            Some(3),
            Some(5),
            Some(1),
            Some(6),
            Some(2),
            Some(0),
            Some(8),
            None,
            None,
            Some(7),
            Some(4),
        ]);
        let p = to_tree(vec![
            Some(5),
            Some(6),
            Some(2),
            None,
            None,
            Some(7),
            Some(4),
        ]);
        let q = to_tree(vec![Some(4)]);
        assert_eq!(
            Solution::lowest_common_ancestor(root, p.clone(), q),
            // to_tree(vec![Some(5)])
            p.clone()
        );
    }
}
