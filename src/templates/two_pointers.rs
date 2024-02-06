use super::CONDITION;

fn one_input_opposite_ends(arr: &[i32]) -> i32 {
    let mut left = 0;
    let mut ans = 0;
    let mut right = arr.len() as isize - 1;

    while left < right {
        // do some logic here with left and right
        if CONDITION {
            left += 1;
        } else {
            right -= 1;
        }
    }

    ans
}

fn two_inputs_exhaust_both(arr1: &[i32], arr2: &[i32]) -> i32 {
    let mut i = 0;
    let mut j = 0;
    let mut ans = 0;

    while i < arr1.len() && j < arr2.len() {
        // do some logic here
        if CONDITION {
            i += 1;
        } else {
            j += 1;
        }
    }

    while i < arr1.len() {
        // do logic
        i += 1;
    }

    while j < arr2.len() {
        // do logic
        j += 1;
    }

    ans
}
