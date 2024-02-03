# Array Rotation

Array rotation is the process of moving the elements of an array to the left or right by a certain number of positions. There are various ways to rotate an array, including methods like the reversal algorithm, the block swap algorithm, and using a temporary array.

Basically, given an array, the task is to cyclically rotate the array clockwise by n.

## Array Rotation with a loop

### n = 1

Algorithm

- Store the last element in a temp variable
- Shift all elements one position to the right
- Store the temp variable in the first position

```rust
fn rotate_array(arr: &[i32]) -> Vec<i32> {
    // Create a temp variable to store the last element
    let temp = arr[arr.len() - 1];

    // Iterate over the array in reverse order
    for i in (1..arr.len()).rev() {
        // Shift all elements one position to the right
        arr[i] = arr[i - 1];
    }

    // Store the temp variable in the first position
    arr[0] = temp;
}
```

The idiomatic approach in Rust is to use the `pop` and `insert` methods on the `Vec` type.

```rust
fn rotate_array(arr: &[i32]) -> Vec<i32> {
    // Create a mutable copy of the input array
    let mut rotated = arr.to_vec();
    // Store the last element in a temp variable
    let temp = rotated.pop().unwrap();
    // Shift all elements one position to the right
    rotated.insert(0, temp);
    // Return the rotated array
    rotated
}
```

### n > 1

Algorithm

- Store the last n elements in a temp variable
- Shift all elements n positions to the right
- Store the temp variable in the first n positions

```rust
fn rotate_array(arr: &[i32], n: usize) -> Vec<i32> {
    // Create a mutable copy of the input array
    let mut rotated = arr.to_vec();
    // Store the last n elements in a temp variable
    let temp = rotated.split_off(rotated.len() - n);
    // Shift all elements n positions to the right
    rotated.splice(0..0, temp);
    // Return the rotated array
    rotated
}
```

You can also use the `rotate_right` method on the `Vec` type.

```rust

fn rotate_array(arr: &[i32], n: usize) -> Vec<i32> {
    // Create a mutable copy of the input array
    let mut rotated = arr.to_vec();
    // Rotate the array n positions to the right
    rotated.rotate_right(n);
    // Return the rotated array
    rotated
}
```

Using the pop and insert method

```rust
fn rotate_array(arr: &[i32], n: usize) -> Vec<i32> {
    // Convert the input slice into a vector to allow modification
    let mut rotated = arr.to_vec();

    // Remove the last `n` elements from the vector and store them in `temp`
    // If the vector is not empty and `n` is less than or equal to the length of the vector, this will succeed
    if let Some(temp) = rotated.split_off(rotated.len() - n).pop() {
        // Insert the last `n` elements at the beginning of the vector
        rotated.splice(0..0, temp);
    }

    // Return the rotated vector, which has been rotated by `n` positions to the right
    rotated
}

```

### Complexity Analysis

The time complexity of the above algorithm is O(n) because we are shifting the elements of the array by n positions.

The space complexity of the above algorithm is O(n) because we are creating a new array to store the rotated array.

## Array Rotation with Two Pointer Technique

The two pointer technique is a method for solving problems by maintaining two pointers that traverse the array. It is often used to solve problems related to arrays and linked lists.

In cyclic rotation we will bring the last element to the first position and shift the rest forward by swapping every element with the last element until we get to the last element.

Algorithm

- Init two pointers, i and j, which point to the first and last element of the array
- Swap the elements at i and j, keep j fixed and i moving towards j
- Repeat the swapping until i is equal to j

```rust
fn rotate_array(arr: &mut [i32]) {
    let (mut i, mut j) = (0, arr.len() - 1);
    while i < j {
        arr.swap(i, j);
        i += 1;
    }
}
```

### Complexity Analysis

The time complexity of the above algorithm is O(n) because we are swapping the elements of the array.

The space complexity of the above algorithm is O(1) because we are not using any extra space.

## Array Rotation with Reversal Algorithm

The reversal algorithm is a method for rotating an array by reversing the elements of the array in a certain order.

Algorithm

- Reverse the array twice
- First time, we will reverse the first n-1 elements
- Second time, we will reverse the last n elements

```rust
fn rotate_array(arr: &mut [i32], n: usize) {
    // Reverse the first n-1 elements
    arr[0..n - 1].reverse();
    // Reverse the last n elements
    arr[n..].reverse();
    // Reverse the entire array
    arr.reverse();
}
```

### Complexity Analysis

The time complexity of the above algorithm is O(n) because we are reversing the elements of the array.

The space complexity of the above algorithm is O(1) because we are not using any extra space.
