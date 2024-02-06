fn prefix_sum(arr: &[i32]) -> Vec<i32> {
    let mut prefix = Vec::with_capacity(arr.len());
    if let Some(first) = arr.first() {
        prefix.push(*first);

        for &item in &arr[1..] {
            let last = *prefix.last().unwrap();
            prefix.push(last + item);
        }
    }

    prefix
}
