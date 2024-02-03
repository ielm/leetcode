# Insertion Sort

Insertion sort works the way many people sort a hand of playing cards. We start with an empty left hand and the cards face down on the table. We then remove one card at a time from the table and insert it into the correct position in the left hand. To find the correct position for a card, we compare it with each of the cards already in the hand, from right to left.

```python
def insertion_sort(arr):
    for i in range(1, len(arr)):
        key = arr[i]
        j = i - 1
        while j >= 0 and key < arr[j]:
            arr[j + 1] = arr[j]
            j -= 1
        arr[j + 1] = key
    return arr
```

```rust
fn insertion_sort(arr: &mut [i32]) {
    // Iterate over the array starting from the second element.
    for i in 1..arr.len() {
        // Start from the current index and move backwards.
        let mut j = i;
        // Continue swapping elements while the current element is less than
        // the previous element.
        while j > 0 && arr[j] < arr[j - 1] {
            // Swap the current element with the previous element.
            arr.swap(j, j - 1);
            // Move one position back in the array.
            j -= 1;
        }
    }
}
```

The loop invariant is the set of elements `arr[1..j-1]` that were originally in positions 1 through j-1, but now in sorted order. There are three properties for the loop invariant (the array consisting of elements 1..j-1):

1. Initialization: It is true prior to the first iteration of the loop.
2. Maintenance: If it is true before an iteration of the loop, it remains true before the next iteration.
3. Termination: When the loop terminates, the invariant gives us a useful property that helps show that the algorithm is correct.

## The non-increasing order

```python
def insertion_sort(arr):
    for in in range(1, len(arr)):
        key = arr[i]
        j = i - 1
        while j > 0 and key[i] > arr[j]:
            arr[j + 1] = arr[j]
            j -= 1
        arr[j + 1] = key
    return arr
```

```rust
fn insertion_sort(arr: &mut [i32]) {
    // Iterate over the array starting from the second element.
    for i in 1..arr.len() {
        let mut j = i;
        // Continue moving elements while the current element is greater than
        // the previous element, which will sort the array in non-increasing order.
        while j > 0 && arr[j] > arr[j - 1] {
            // Swap the current element with the previous element.
            arr.swap(j, j - 1);
            // Move one position back in the array.
            j -= 1;
        }
    }
}
```

## Linear Search Loop Invariant

For the linear search algorithm, the loop invariant can be stated as follows:

At the start of each iteration of the for-loop from `0` to `arr.len() - 1`, none of the elements in the subarray from `A[0]` to `A[i-1]` is equal to the value `v` that we are searching for.

```rust
fn linear_search(arr: &[i32], v: i32) -> Option<usize> {
    for (i, &item) in arr.iter().enumerate() {
        if item == v {
            return Some(i);
        }
    }
    None
}
```

Here are the properties of the loop invariant:

1. Initialization: Before the loop starts, the subarray A[0..0] (which is empty) trivially contains no element equal to `v`.
2. Maintenance: If `A[i]` is not equal to `v`, then the invariant is maintained as `A[i]` is added to the subarray of elements that do not match `v`. The search continues to the next element.
3. Termination: The loop terminates in two cases:
   a. If an element equal to `v` is found, in which case the function returns the index of that element.
   b. If the end of the array is reached without finding `v`, in which case the invariant tells us that `v` is not present in the array at all, and the function returns `None`.

## Adding two n-bit binary integers

Consider the problem of adding two n-bit binary integers, stored in two n-element arrays A and B. The sum of the two integers should be stored in binary form in an (n + 1)-element array C. State the problem formally and write pseudocode for adding the two integers.

Problem statement

Given two n-bit binary integers `A` and `B`, the problem is to add them together and store the sum in an (n + 1)-bit binary integer `C`.

```rust

fn add_binary_integers(a: &[u8], b: &[u8]) -> Vec<u8> {
    // Initialize the result vector with an extra bit for potential carry.
    let mut c = vec![0; a.len() + 1];
    // Initialize the carry to zero.
    let mut carry = 0;

    // Iterate over the bits of a and b in reverse order (from least significant to most significant bit).
    for i in (0..a.len()).rev() {
        // Calculate the sum of the corresponding bits and the carry.
        let sum = a[i] + b[i] + carry;
        // The ith bit of the result is the remainder of the sum divided by 2 (binary addition).
        c[i + 1] = sum % 2;
        // Update the carry to be the quotient of the sum divided by 2 (binary addition).
        carry = sum / 2;
    }

    // After the loop, if there's a remaining carry, it becomes the most significant bit of the result.
    c[0] = carry;

    // Return the resulting binary number as a vector of bits.
    c
}

```
