/**
 * [227] Basic Calculator II
 *
 * Given a string s which represents an expression, evaluate this expression and return its value.
 * The integer division should truncate toward zero.
 * You may assume that the given expression is always valid. All intermediate results will be in the range of [-2^31, 2^31 - 1].
 * Note: You are not allowed to use any built-in function which evaluates strings as mathematical expressions, such as eval().
 *  
 * <strong class="example">Example 1:
 * Input: s = "3+2*2"
 * Output: 7
 * <strong class="example">Example 2:
 * Input: s = " 3/2 "
 * Output: 1
 * <strong class="example">Example 3:
 * Input: s = " 3+5 / 2 "
 * Output: 5
 *  
 * Constraint
 *
 *      1 <= s.length <= 3 * 10^5
 *      s consists of integers and operators ('+', '-', '*', '/') separated by some number of spaces.
 *      s represents a valid expression.
 *      All the integers in the expression are non-negative integers in the range [0, 2^31 - 1].
 *      The answer is guaranteed to fi in a 32-bit integer.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/basic-calculator-ii/
// discuss: https://leetcode.com/problems/basic-calculator-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let length = s.len();

        if length == 0 {
            return 0;
        }

        let (mut curr_num, mut last_num, mut res) = (0, 0, 0);

        let mut sign = '+';

        for (i, c) in s.chars().enumerate() {
            if c.is_ascii_digit() {
                curr_num = curr_num * 10 + c.to_digit(10).unwrap() as i32;
            }
            if !c.is_ascii_digit() && !c.is_whitespace() || i == length - 1 {
                if sign == '+' || sign == '-' {
                    res += last_num;
                    last_num = if sign == '+' { curr_num } else { -curr_num };
                } else if sign == '*' {
                    last_num *= curr_num;
                } else if sign == '/' {
                    last_num /= curr_num;
                }
                sign = c;
                curr_num = 0;
            }
        }
        res + last_num
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_227_1() {
        let s = "3+2*2".to_string();
        let result = 7;
        assert_eq!(Solution::calculate(s), result);
    }

    #[test]
    fn test_227_2() {
        let s = " 3/2 ".to_string();
        let result = 1;
        assert_eq!(Solution::calculate(s), result);
    }

    #[test]
    fn test_227_3() {
        let s = " 3+5 / 2 ".to_string();
        let result = 5;
        assert_eq!(Solution::calculate(s), result);
    }
}
