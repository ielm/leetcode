use super::CONDITION;

#[allow(unused_variables)]
#[allow(unused_mut)]
fn sliding_window(arr: &mut Vec<i32>) -> i32 {
    let mut left = 0;
    let mut ans = 0;
    let mut curr = 0;

    for _right in 0..arr.len() {
        // do logic here to add arr[right] to curr

        while CONDITION {
            // remove arr[left] from curr
            left += 1;
        }

        // update ans
    }

    ans
}
