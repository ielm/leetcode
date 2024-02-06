use std::collections::HashMap;

pub fn num_subarrays_matching_criteria(arr: &[i32], k: i32) -> i32 {
    let mut counts = HashMap::new();
    counts.insert(0, 1);
    let mut ans = 0;
    let mut curr = 0;

    for &num in arr {
        // do logic to change curr
        curr += num; // Assuming the logic is to accumulate the values as in C++ example
        ans += counts.get(&(curr - k)).unwrap_or(&0);
        *counts.entry(curr).or_insert(0) += 1;
    }

    ans
}
