# Breadth First Search


## [Binary Tree Vertical Order Traversal](https://leetcode.com/problems/binary-tree-vertical-order-traversal/)

This is a problem about binary tree traversals. There are two strategies to traverse a tree data structure - depth first search and breadth first search.

In this problem, we're asked to return the vertical order of a binary tree, which implies two sub-orders, where each node would have a 2D index denoted as `<column, row>`

### Column-wise order

If we look at the binary tree horizontally, each node can be aligned to a column based on its relative offset to the root node of the tree.

If we assume that the root node has a column index of 0, then its left child node would have a column index of -1, and its right child node would have a column index of +1, and so on.

### Row-wise order

If we put the nodes into a vertical dimension, each node would be assigned to a row based on it's level = the vertical distance from the root node.

If we assume that the root node has a row index of 0, then its child nodes would have a row index of 1, and so on.

Nodes should be ordered by column first, and then the nodes on the same column should be ordered vertically based on their row indices.

### Approach 1: Breadth First Search

The first approach to solve this problem is to use breadth first search to traverse the tree and assign the column and row indices to each node.

With the BFS traversal, we naturally can guarantee the vertical order of the visits - the nodes at higher row indices would get visited later than the ones at lower row indices.

We would still be missing the horizontal (column) order. To solve this, we can use a hash map to store the nodes based on their column indices. `columnTable<col_index, node_list>`.

The key in the hash table would be the column index, and the corresponding value would be a list of nodes that have the same column index, where the values should be ordered by their row indices 0 guaranteed by the BFS traversal.

#### Algorithm

1. Create a hash map `columnTable` to store the nodes based on their column indices.
2. Create a queue to store the nodes to be visited. We initialize the queue by adding the root node with a column index of 0.
3. We run the BFS traversal with a loop consuming the elements from the queue.
4. At each iteration in the BFS, we pop out an element from the queue. The element consists of a node and it's column index. If the node is not empty, we populate the columnTable with the node's column index and the node itself. We then add the left and right child nodes to the queue with their respective column indices (col-1 and col+1).
5. At the end of the BFS traversal, we obtain a hash table that contains the desired node values grouped by their column indices. For each group of values, they are further ordered by their row indices.
6. We then sort the hash table by its keys (column indices) and return the values as the result.

#### Complexity Analysis
Time complexity: O(NlogN) where N is the number of nodes in the tree. The time complexity is dominated by the sorting operation of the hash table.

The first part of the time complexity is O(N) because we need to visit each node in the tree to populate the hash table.

The second part of the time complexity is O(NlogN) because we need to sort the hash table by its keys (column indices).

Space complexity: O(N) where N is the number of nodes in the tree. The space complexity is dominated by the hash table and the queue used in the BFS traversal.
