// The same logic can be applied to maintain a monotonic queue

pub fn monotonic_increasing_stack(arr: &[i32]) {
    let mut stack: Vec<i32> = Vec::new();
    let mut ans = 0;

    for &num in arr {
        while let Some(&top) = stack.last() {
            // For monotonic decreasing stack, change > to < in the next line
            if top > num {
                // do logic
                stack.pop();
            } else {
                break;
            }
        }

        stack.push(num);
    }
}
