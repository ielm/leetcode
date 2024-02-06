/**
 * [339] Nested List Weight Sum - PREMIUM PROBLEM
 *
 * You are given a nested list of integers nestedList. Each element is either an integer or a list whose elements may also be integers or other lists.
 * The depth of an integer is the number of lists that it is inside of. For example, the nested list [1,[2,2],[[3],2],1] has each integer's value set to its depth.
 * Return the sum of each integer in nestedList multiplied by its depth.
 *
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/14/nestedlistweightsumex1.png" style="width: 405px; height: 99px;" />
 * Input: nestedList = [[1,1],2,[1,1]]
 * Output: 10
 * Explanation: Four 1's at depth 2, one 2 at depth 1. 1*2 + 1*2 + 2*1 + 1*2 + 1*2 = 10.
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/14/nestedlistweightsumex2.png" style="width: 315px; height: 106px;" />
 * Input: nestedList = [1,[4,[6]]]
 * Output: 27
 * Explanation: One 1 at depth 1, one 4 at depth 2, and one 6 at depth 3. 1*1 + 4*2 + 6*3 = 27.
 * <strong class="example">Example 3:
 *
 * Input: nestedList = [0]
 * Output: 0
 *
 *
 * Constraints:
 *
 *     1 <= nestedList.length <= 50
 *     The values of the integers in the nested list is in the range [-100, 100].
 *     The maximum depth of any integer is less than or equal to 50.
 *
 */
use crate::util::nested_integer::NestedInteger;

pub struct Solution {}

// problem: https://leetcode.com/problems/nested-list-weight-sum/
// discuss: https://leetcode.com/problems/nested-list-weight-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn depth_sum(nested_list: Vec<NestedInteger>) -> i32 {
        Self::dfs(nested_list, 1)
    }

    pub fn dfs(nested_list: Vec<NestedInteger>, depth: i32) -> i32 {
        // The total weighted sum so far
        let mut total = 0;

        for item in nested_list {
            match item {
                NestedInteger::Int(val) => {
                    total += val * depth;
                }
                NestedInteger::List(list) => {
                    total += Self::dfs(list, depth + 1);
                }
            }
        }
        total
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_339_1() {
        assert_eq!(
            Solution::depth_sum(vec![
                NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1)]),
                NestedInteger::Int(2),
                NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1)])
            ]),
            10
        );
    }

    #[test]
    fn test_339_2() {
        assert_eq!(
            Solution::depth_sum(vec![
                NestedInteger::Int(1),
                NestedInteger::List(vec![
                    NestedInteger::Int(4),
                    NestedInteger::List(vec![NestedInteger::Int(6)])
                ])
            ]),
            27
        );
    }

    #[test]
    fn test_339_3() {
        assert_eq!(Solution::depth_sum(vec![NestedInteger::Int(0)]), 0);
    }
}
