// For graph templates, assume the nodes are numbered from `0` to `n - 1`
// and the graph is given as an adjacency list `graph` of size `n`, where
// `adj[i]` is a list of nodes connected to node `i`.
// Depending on the problem, you may need to convert the input into an equivalent
// adjacency list before using the templates.
//
// Adjacency list looks like the following:
//
// graph = [
//    [1, 2], // Node 0 is connected to nodes 1 and 2.
//    [0],    // Node 1 is connected to node 0.
//    [0],    // Node 2 is connected to node 0.
// ]
//

use crate::util::graph::GraphNode;
use std::cell::RefCell;
use std::collections::{HashSet, VecDeque};
use std::rc::Rc;

type AdjList = Vec<Vec<usize>>;
type ValAdjList = Vec<(i32, Vec<usize>)>;

struct AdjListTemplates {}

impl AdjListTemplates {
    fn recursive_dfs(graph: &AdjList, start_node: usize) -> i32 {
        // Initialize a set to keep track of the seen nodes.
        let mut seen = HashSet::new();
        // Mark the start node as seen.
        seen.insert(start_node);
        // Start the depth-first search from the start node.
        Self::_recursive_dfs(start_node, graph, &mut seen)
    }

    fn _recursive_dfs(node: usize, graph: &AdjList, seen: &mut HashSet<usize>) -> i32 {
        // Initialize a counter to keep track of some result, e.g., node count, path length.
        let mut ans = 0;
        // Iterate over the neighbors of the current node.
        for &neighbor in &graph[node] {
            // If the neighbor hasn't been seen, mark it as seen and explore it recursively.
            if seen.insert(neighbor) {
                ans += Self::_recursive_dfs(neighbor, graph, seen);
            }
        }
        // Return the result accumulated during the DFS.
        ans
    }

    fn iterative_dfs(graph: &AdjList, start_node: usize) -> i32 {
        // Initialize a set to keep track of the seen nodes to avoid revisiting them.
        let mut seen = HashSet::new();
        // Mark the start node as seen to prevent it from being revisited.
        seen.insert(start_node);
        // Initialize a stack to keep track of the nodes to visit in LIFO order.
        let mut stack = Vec::new();
        // Start with the start_node by pushing it onto the stack.
        stack.push(start_node);

        // Initialize a counter to keep track of some result, e.g., node count.
        let mut ans = 0;

        // Continue until there are no more nodes to visit.
        while let Some(node) = stack.pop() {
            // Increment the result counter for each node visited.
            ans += 1;
            // Iterate over the neighbors of the current node.
            for &neighbor in &graph[node] {
                // If the neighbor hasn't been seen, mark it as seen and add it to the stack.
                if seen.insert(neighbor) {
                    stack.push(neighbor);
                }
            }
        }
        // Return the accumulated result, which could represent the number of nodes visited.
        ans
    }

    fn bfs(graph: &AdjList, start_node: usize) -> i32 {
        // Initialize a set to keep track of the seen nodes to avoid revisiting them.
        let mut seen = HashSet::new();
        // Mark the start node as seen to prevent it from being revisited.
        seen.insert(start_node);
        // Initialize a queue to keep track of the nodes to visit in FIFO order.
        let mut queue = VecDeque::new();
        // Start with the start_node by pushing it onto the queue.
        queue.push_back(start_node);

        // Initialize a counter to keep track of some result, e.g., node count.
        let mut ans = 0;

        // Continue until there are no more nodes to visit.
        while let Some(node) = queue.pop_front() {
            // Increment the result counter for each node visited.
            ans += 1;
            // Iterate over the neighbors of the current node.
            for &neighbor in &graph[node] {
                // If the neighbor hasn't been seen, mark it as seen and add it to the queue.
                if seen.insert(neighbor) {
                    queue.push_back(neighbor);
                }
            }
        }
        // Return the accumulated result, which could represent the number of nodes visited.
        ans
    }
}

struct ValAdjListTemplates {}

impl ValAdjListTemplates {
    fn recursive_dfs(graph: &ValAdjList, start_node: usize) -> i32 {
        // Initialize a set to keep track of the seen nodes.
        let mut seen = HashSet::new();
        // Mark the start node as seen.
        seen.insert(start_node);
        // Start the depth-first search from the start node.
        Self::_recursive_dfs(start_node, graph, &mut seen)
    }

    fn _recursive_dfs(node: usize, graph: &ValAdjList, seen: &mut HashSet<usize>) -> i32 {
        // Initialize a counter to keep track of some result, e.g., node count, path length.
        let mut ans = graph[node].0;
        // Iterate over the neighbors of the current node, which are tuples of (value, node).
        for neighbor in &graph[node].1 {
            // If the neighbor hasn't been seen, mark it as seen and explore it recursively.
            if seen.insert(*neighbor) {
                ans += Self::_recursive_dfs(*neighbor, graph, seen);
            }
        }
        ans
    }

    fn iterative_dfs(graph: &ValAdjList, start_node: usize) -> i32 {
        // A set to keep track of the nodes that have already been visited.
        let mut seen = HashSet::new();
        // Mark the starting node as seen to avoid revisiting it.
        seen.insert(start_node);

        // A stack to manage the nodes to visit next, following LIFO order.
        let mut stack = Vec::new();
        // Push the starting node onto the stack to begin the DFS.
        stack.push(start_node);

        // A counter to accumulate the sum of the values of the visited nodes.
        let mut ans = graph[start_node].0;

        // Continue the DFS until there are no more nodes to visit.
        while let Some(node) = stack.pop() {
            // Iterate over the neighbors of the current node.
            for &neighbor in &graph[node].1 {
                // If the neighbor hasn't been seen, process it.
                if seen.insert(neighbor) {
                    // Add the value of the neighbor to the accumulated sum.
                    ans += graph[neighbor].0;
                    // Push the neighbor onto the stack to visit its neighbors later.
                    stack.push(neighbor);
                }
            }
        }
        // Return the sum of the values of all visited nodes.
        ans
    }

    fn bfs(graph: &ValAdjList, start_node: usize) -> i32 {
        // A set to keep track of the nodes that have already been visited.
        let mut seen = HashSet::new();
        // Mark the starting node as seen to avoid revisiting it.
        seen.insert(start_node);

        // A queue to manage the nodes to visit next, following FIFO order.
        let mut queue = VecDeque::new();
        // Push the starting node onto the queue to begin the BFS.
        queue.push_back(start_node);

        // A counter to accumulate the sum of the values of the visited nodes.
        let mut ans = graph[start_node].0;

        // Continue the BFS until there are no more nodes to visit.
        while let Some(node) = queue.pop_front() {
            // Iterate over the neighbors of the current node.
            for &neighbor in &graph[node].1 {
                // If the neighbor hasn't been seen, process it.
                if seen.insert(neighbor) {
                    // Add the value of the neighbor to the accumulated sum.
                    ans += graph[neighbor].0;
                    // Push the neighbor onto the queue to visit its neighbors later.
                    queue.push_back(neighbor);
                }
            }
        }
        // Return the sum of the values of all visited nodes.
        ans
    }
}

struct GraphNodeTemplates {}

impl GraphNodeTemplates {
    fn recursive_dfs(root: Rc<RefCell<GraphNode>>) -> i32 {
        // Initialize the answer to store the sum of node values
        let mut ans = root.borrow().val;

        // Do Logic

        // Recurse on the neighbors
        for neighbor in &root.borrow().neighbors {
            // Add the result of the recursive call to the answer
            ans += Self::recursive_dfs(neighbor.clone());
        }

        ans
    }

    fn iterative_dfs(root: Rc<RefCell<GraphNode>>) -> i32 {
        // Initialize the answer to store the sum of node values
        let mut ans = root.borrow().val;

        // A stack to manage the nodes to visit next, following LIFO order.
        let mut stack = Vec::new();
        // Push the starting node onto the stack to begin the DFS.
        stack.push(root.clone());

        // Continue the DFS until there are no more nodes to visit.
        while let Some(node) = stack.pop() {
            // Iterate over the neighbors of the current node.
            for neighbor in &node.borrow().neighbors {
                // Add the value of the neighbor to the accumulated sum.
                ans += neighbor.borrow().val;
                // Push the neighbor onto the stack to visit its neighbors later.
                stack.push(neighbor.clone());
            }
        }
        // Return the sum of the values of all visited nodes.
        ans
    }

    fn iterative_bfs(root: Rc<RefCell<GraphNode>>) -> i32 {
        // Initialize the answer to store the sum of node values.
        let mut ans = root.borrow().val;

        // A queue to manage the nodes to visit next, following FIFO order.
        let mut queue = VecDeque::new();
        // Push the starting node onto the queue to begin the BFS.
        queue.push_back(root.clone());

        // Continue the BFS until there are no more nodes to visit.
        while let Some(node) = queue.pop_front() {
            // Iterate over the neighbors of the current node.
            for neighbor in &node.borrow().neighbors {
                // Add the value of the neighbor to the accumulated sum.
                ans += neighbor.borrow().val;
                // Push the neighbor onto the queue to visit its neighbors later.
                queue.push_back(neighbor.clone());
            }
        }

        // Return the sum of the values of all visited nodes.
        ans
    }
}
