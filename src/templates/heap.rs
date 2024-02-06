use std::collections::BinaryHeap;

fn find_top_k_elements(arr: &[i32], k: usize) -> Vec<i32> {
    // Create a binary heap to maintain the top k elements.
    let mut heap = BinaryHeap::new();

    // Iterate over the array and maintain the size of the heap up to k elements.
    for &num in arr {
        // Push the current number onto the heap.
        heap.push(num);
        // If the heap size exceeds k, remove the largest element.
        if heap.len() > k {
            heap.pop();
        }
    }

    // Prepare a vector to store the top k elements in ascending order.
    let mut ans = Vec::with_capacity(k);
    // Extract elements from the heap until we have k elements in the answer vector.
    while let Some(top) = heap.pop() {
        ans.push(top);
        // If the answer vector is full, stop the loop.
        if ans.len() >= k {
            break;
        }
    }

    // The BinaryHeap is a max-heap by default, so we need to reverse the elements
    // to get the top k elements in ascending order.
    ans.reverse();
    ans
}
