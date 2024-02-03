/**
 * [109] Convert Sorted List to Binary Search Tree
 *
 * Given the head of a singly linked list where elements are sorted in ascending order, convert it to a <span data-keyword="height-balanced">height-balanced</span> binary search tree.
 *
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/08/17/linked.jpg" style="width: 500px; height: 388px;" />
 * Input: head = [-10,-3,0,5,9]
 * Output: [0,-3,9,-10,null,5]
 * Explanation: One possible answer is [0,-3,9,-10,null,5], which represents the shown height balanced BST.
 *
 * <strong class="example">Example 2:
 *
 * Input: head = []
 * Output: []
 *
 *
 * Constraints:
 *
 *     The number of nodes in head is in the range [0, 2 * 10^4].
 *     -10^5 <= Node.val <= 10^5
 *
 */
// problem: https://leetcode.com/problems/convert-sorted-list-to-binary-search-tree/
// discuss: https://leetcode.com/problems/convert-sorted-list-to-binary-search-tree/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
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

// This can be solved using the Day-Stout-Warren algorithm, which consists of a
// main routine with three subroutines.
// The main routine is given by:
// 1. Allocate a node, the "pseudo-root" of the tree, and make the tree's actual root
//    the right child of the pseudo-root.
// 2. Call tree-to-vine on the pseudo-root.
// 3. Call vine-to-tree on the pseudo-root and the size of the tree (num elements)
// 4. Make the tree's actual root the right child of the pseudo-root.
// 5. Dispose of the pseudo-root.

// The tree-to-vine subroutine is given by: tree_to_vine(root: Node)
// 1. Make the root the tail of the vine.
// 2. Make the rest the tail.right
// 3. While rest is not null,
//    a. if rest.left is null
//       a.1 make the tail the rest
//       a.2 make rest the rest.right
//    b. else
//       make temp the rest.left
//       make rest.left the temp.right
//       make temp.right the rest
//       make rest the temp
//       make tail.right the temp

// The vine-to-tree subroutine is given by: vine_to_tree(root: Node, size: i32)
// 1. let leaves = size + 1 - 2^floor(log2(size + 1))
// 2. compress(root, leaves)
// 3. let size = size - leaves
// 4. while size > 1
//    a. compress(root, size / 2)
//    b. size = size / 2

// The compress subroutine is given by: compress(subroot: Node, count: i32)
// 1. let scanner = subroot
// 2. for _ in 0..count
//    a. let child = scanner.right
//    b. scanner.right = child.right
//    c. scanner = scanner.right
//    d. child.right = scanner.left
//    e. scanner.left = child
use std::cell::RefCell;
use std::rc::Rc;

use crate::util::linked_list::{to_list, ListNode};
use crate::util::tree::{to_tree, TreeNode};

pub struct RecursiveSolution {}
impl RecursiveSolution {
    /// Converts a sorted linked list to a height-balanced binary search tree.
    ///
    /// The function works recursively, first counting the length of the linked list,
    /// then building the tree top-down by selecting the median of the list as the root,
    /// and recursively constructing the left and right subtrees.
    ///
    /// # Arguments
    /// * `head`: An `Option` that holds the head of the linked list.
    ///
    /// # Returns
    /// An `Option` containing the root of the height-balanced binary search tree.
    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        // First, calculate the length of the linked list
        let mut len = 0;
        let mut node = &head;
        while let Some(n) = node {
            len += 1;
            node = &n.next;
        }

        // Create a mutable reference to the head of the list
        let mut head = head;

        // Recursively build the BST from the linked list
        Self::dfs(&mut head, len)
    }

    /// Recursive helper function to construct the BST from the linked list.
    ///
    /// # Arguments
    /// * `list`: A mutable reference to the current head of the linked list.
    /// * `len`: The length of the remaining linked list.
    ///
    /// # Returns
    /// An `Option` containing the current `TreeNode` being constructed.
    fn dfs(list: &mut Option<Box<ListNode>>, len: usize) -> Option<Rc<RefCell<TreeNode>>> {
        // Base case: if the list is empty, return None
        if len == 0 {
            return None;
        }

        // Recursively construct the left subtree
        let left = Self::dfs(list, len / 2);

        // Create the current TreeNode with the value from the head of the list
        if let Some(head) = list {
            let mut node = TreeNode::new(head.val);
            // Move the list head to the next element
            *list = head.next.take();
            // Set the left child of the TreeNode
            node.left = left;
            // Recursively construct the right subtree and set it as the right child
            node.right = Self::dfs(list, len - len / 2 - 1);
            // Wrap the TreeNode in Rc and RefCell for shared ownership and mutability
            Some(Rc::new(RefCell::new(node)))
        } else {
            None
        }
    }
}

pub struct DwsSolution {}

impl DwsSolution {
    pub fn sorted_list_to_bst(mut head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        // Create a pseudo-root node with a dummy value
        let pseudo = Rc::new(RefCell::new(TreeNode::new(-1)));
        // This will keep track of the current node while converting the list to a vine
        let mut curr = pseudo.clone();
        // Count the number of nodes in the list
        let mut count = 0;

        // Step 1: Convert the linked list to a vine (a right-skewed tree)
        while let Some(mut node) = head {
            // Detach the current head node from the list
            head = node.next.take();
            // Create a new tree node with the detached list node's value
            let new_treenode = Rc::new(RefCell::new(TreeNode::new(node.val)));
            // Attach the new tree node as the right child of the current node
            curr.borrow_mut().right = Some(new_treenode.clone());
            // Move to the new tree node
            curr = new_treenode;
            // Increment the count of nodes processed
            count += 1;
        }

        // Step 2: Convert the vine into a balanced BST
        while count > 1 {
            curr = pseudo.clone();
            // Check if count is not a power of 2 minus 1 (all 1s in binary)
            while ((count + 1) & count) != 0 {
                // Perform rotations to balance the BST
                curr = Self::rotate(curr.clone());
                // Decrease the count after each rotation
                count -= 1;
            }
            // Decrease the count for the last set of nodes
            count -= 1;
        }

        // The right child of the pseudo-root now points to the root of the balanced BST
        let res = pseudo.borrow_mut().right.take();
        res
    }

    /// Performs a right rotation at the current node and returns the new parent node.
    ///
    /// # Arguments
    /// * `curr`: The current node wrapped in `Rc<RefCell<TreeNode>>`.
    ///
    /// # Returns
    /// * The new parent node after the rotation, wrapped in `Rc<RefCell<TreeNode>>`.
    fn rotate(curr: Rc<RefCell<TreeNode>>) -> Rc<RefCell<TreeNode>> {
        // Detach the right child of the current node
        let c = curr.borrow_mut().right.take().unwrap();
        // Detach the right child of the detached node (c)
        let d = c.borrow_mut().right.take().unwrap();

        // Perform a right rotation
        c.borrow_mut().right = d.borrow_mut().left.take();
        d.borrow_mut().left = Some(c);

        // Attach the new subtree as the right child of the current node
        curr.borrow_mut().right = Some(d.clone());

        // Return the new parent node after rotation
        d
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_109() {}
}
