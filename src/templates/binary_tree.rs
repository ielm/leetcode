use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::util::tree::TreeNode;

/// Recursive depth-first search function that traverses a binary tree and
/// computes the sum of the values of all nodes.
///
/// Parameters:
/// - root: an `Rc<RefCell<TreeNode>>` representing the root of the binary tree.
///
/// Returns:
/// - The sum of the values of all nodes in the tree.
fn recursive_dfs(root: Rc<RefCell<TreeNode>>) -> i32 {
    let mut ans = 0; // Initialize the answer to store the sum of node values.

    // Do Logic

    // If the current node has a left child, recursively call the function
    // on the left subtree and add the result to the answer.
    if let Some(ref left) = root.borrow().left {
        ans += recursive_dfs(left.clone());
    }

    // If the current node has a right child, recursively call the function
    // on the right subtree and add the result to the answer.
    if let Some(ref right) = root.borrow().right {
        ans += recursive_dfs(right.clone());
    }

    ans // Return the computed sum.
}

/// Iterative depth-first search function that traverses a binary tree and
/// computes the sum of the values of all nodes using a stack.
///
/// Parameters:
/// - root: an `Rc<RefCell<TreeNode>>` representing the root of the binary tree.
///
/// Returns:
/// - The sum of the values of all nodes in the tree.
fn iterative_dfs(root: Rc<RefCell<TreeNode>>) -> i32 {
    let mut ans = 0; // Initialize the answer to store the sum of node values.
    let mut stack = vec![root.clone()]; // Initialize a stack with the root node.

    // While there are nodes on the stack, process each node.
    while let Some(node) = stack.pop() {
        // Add the current node's value to the answer.
        ans += node.borrow().val;

        // If the current node has a right child, push it onto the stack.
        if let Some(ref right) = node.borrow().right {
            stack.push(right.clone());
        }

        // If the current node has a left child, push it onto the stack.
        if let Some(ref left) = node.borrow().left {
            stack.push(left.clone());
        }
    }

    ans // Return the computed sum.
}

/// Iterative breadth-first search function that traverses a binary tree level by level
/// and computes the sum of the values of all nodes using a queue.
///
/// Parameters:
/// - root: an `Rc<RefCell<TreeNode>>` representing the root of the binary tree.
///
/// Returns:
/// - The sum of the values of all nodes in the tree.
fn iterative_bfs(root: Rc<RefCell<TreeNode>>) -> i32 {
    let mut ans = 0; // Initialize the answer to store the sum of node values.
    let mut queue = VecDeque::new(); // Initialize a queue to hold nodes at each level.
    queue.push_back(root.clone()); // Start with the root node in the queue.

    // Process nodes level by level until the queue is empty.
    while let Some(node) = queue.pop_front() {
        // Add the current node's value to the answer.
        ans += node.borrow().val;

        // If the current node has a left child, enqueue it for processing.
        if let Some(ref left) = node.borrow().left {
            queue.push_back(left.clone());
        }

        // If the current node has a right child, enqueue it for processing.
        if let Some(ref right) = node.borrow().right {
            queue.push_back(right.clone());
        }
    }

    ans // Return the computed sum.
}
