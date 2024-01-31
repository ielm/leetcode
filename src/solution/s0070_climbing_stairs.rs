/**
 * [70] Climbing Stairs
 *
 * You are climbing a staircase. It takes n steps to reach the top.
 * Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?
 *  
 * <strong class="example">Example 1:
 *
 * Input: n = 2
 * Output: 2
 * Explanation: There are two ways to climb to the top.
 * 1. 1 step + 1 step
 * 2. 2 steps
 *
 * <strong class="example">Example 2:
 *
 * Input: n = 3
 * Output: 3
 * Explanation: There are three ways to climb to the top.
 * 1. 1 step + 1 step + 1 step
 * 2. 1 step + 2 steps
 * 3. 2 steps + 1 step
 *
 *  
 * Constraints:
 *
 * 1 <= n <= 45
 *
 */
// problem: https://leetcode.com/problems/climbing-stairs/
// discuss: https://leetcode.com/problems/climbing-stairs/discuss/?currentPage=1&orderBy=most_votes&query=
use std::cmp::Ordering;

pub struct Solution {}

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut memo: Vec<Option<i32>> = vec![None; n as usize + 1];
        Self::solve(n, &mut memo)
    }

    fn solve(n: i32, memo: &mut Vec<Option<i32>>) -> i32 {
        let i = n as usize;

        match n.cmp(&0) {
            Ordering::Less => 0,
            Ordering::Equal => 1,
            Ordering::Greater => {
                if memo[i].is_some() {
                    memo[i].unwrap()
                } else {
                    memo[i] = Some(Self::solve(n - 1, memo) + Self::solve(n - 2, memo));
                    memo[i].unwrap()
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_70() {
        let input = 3;
        let expected = 3;
        let actual = Solution::climb_stairs(input);

        assert_eq!(expected, actual);

        let input = 2;
        let expected = 2;
        let actual = Solution::climb_stairs(input);

        assert_eq!(expected, actual);
    }
}
