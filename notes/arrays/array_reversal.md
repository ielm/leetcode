# Array Reversal

Reversing an array means shifting the elements of an array in a reverse manner. The time complexity of reversing an array is O(n).

There are various ways to reverse an array, including methods like the two pointer technique, recursion, and using a stack.

## Array Reversal using an extra array

Algorithm

- Create an empty array `B` of the same size as array `A`.
- Iterate over the elements of array `A` in reverse order and copy them to array `B`.

```rust
fn reverse_array(arr: &[i32]) -> Vec<i32> {
    // Create the revesed array with the same size as the original array.
    let mut reversed = vec![0; arr.len()];
    // Iterate over the elements of the original array
    for (i, &item) in arr.iter().enumerate() {
        // for each element index i, copy the element to the reversed array at
        // index arr.len() - 1 - 1
        reversed[arr.len() - 1 - i] = item;
    }
    // Return the reversed array
    reversed
}
```

In Rust, this can also be done functionally in-line:

```rust
fn reverse_array(arr: &[i32]) -> Vec<i32> {
    arr.iter().rev().cloned().collect()
}
```

### Complexity Analysis

The time complexity of this algorithm is O(n), because copying elements to a new array is a linear operation.

The space complexity of this algorithm is O(n), because we are using an extra array of the same size as the original array.

## In place array reversal using a loop (Two Pointer Technique)

The Two Pointer technique is a clever method of keeping track of two pointers, or cursors, that traverse an array. This technique is often used to solve problems that involve searching, sorting, or reversing an array.

Algorithm

- Iterate through the array using two pointers: start and end
- Swap elements at the start and end pointers
- Increment the start pointer and decrement the end pointer until they meet or cross

```rust
fn reverse_array(arr: &[i32]) -> Vec<i32> {
    // Create a mutable copy of the input array
    let mut reversed = arr.to_vec();
    // Initialize the start and end pointers
    let (mut start, mut end) = (0, arr.len() - 1);
    // Iterate through the array using two pointers
    while start < end {
        // Swap the elements at the start and end pointers
        reversed.swap(start, end);
        // Increment the start pointer
        start += 1;
        // Decrement the end pointer
        end -= 1;
    }
}
```

### Complexity Analysis

This algorithm reverses the array in place, without using an extra array, and is more efficient than the previous method.

The time complexity of this algorithm is O(n), because we iterate through the array once.

The space complexity of this algorithm is O(1), because we are not using any extra space.

## Recursive Array Reversal

Algorithm

- Define a recursive function that takes an array as input
- Swap the first and last elements of the array
- Recur on the subarray that excludes the first and last elements

```rust
fn reverse_array(arr: &mut [i32]) {
    // Base case: if the array is empty or has only one element, return
    if arr.len() <= 1 {
        return;
    }
    // Swap the first and last elements of the array
    arr.swap(0, arr.len() - 1);
    // Recur on the subarray that excludes the first and last elements
    reverse_array(&mut arr[1..arr.len() - 1]);
}
```

This can also be done by explicitly defining the start and end indices:

```rust
fn reverse_array(arr: &[i32], start: i32, end: i32) {
    if start >= end {
        return;
    }
    arr.swap(start, end);
    reverse_array(arr, start + 1, end - 1);
}
```

### Complexity Analysis

The time complexity of this algorithm is O(n), because we iterate through the array once.

The space complexity of this algorithm is O(log n), because the recursive calls use the call stack.

## Array Reversal using a Stack

Stacks are a useful data structure for a manner of problems, including reversing an array. The idea of a stack is to use the Last-In-First-Out (LIFO) principle to reverse the order of elements.

Algorithm

- Push each element of the array onto a stack, in order
- Pop each element from the stack to form the reversed array

```rust
fn reverse_array(arr: &[i32]) -> Vec<i32> {
    // Create a stack - in this case we will use a Vec as a stack, but we could also
    // use a linked list or a custom stack implementation. In this case it is better
    // to use a Vec because it has better throughput, has additional capacity
    // (guaranteeing non-alocating push-back as long as there's excess capacity), and
    // maintains amortized O(1) push and pop operations even
    // when not reserving capacity in advance.
    let mut stack = Vec::new();

    // Iterate over array and push back each element onto the stack
    for &item in arr {
        stack.push(item);
    }

    // Return the reversed array by popping each element from the stack
    stack
        .iter()
        .map(
            |&item|
            stack.pop().unwrap())
        .collect()
}
```

### Complexity Analysis

The time complexity of this algorithm is O(n), because we iterate through the array once.

The space complexity of this algorithm is O(n), because we are using a stack to store the elements of the array
