/**
 * [1762] Buildings With an Ocean View
 *
 * There are n buildings in a line. You are given an integer array heights of size n that represents the heights of the buildings in the line.
 * The ocean is to the right of the buildings. A building has an ocean view if the building can see the ocean without obstructions. Formally, a building has an ocean view if all the buildings to its right have a smaller height.
 * Return a list of indices (0-indexed) of buildings that have an ocean view, sorted in increasing order.
 *  
 * <strong class="example">Example 1:
 *
 * Input: heights = [4,2,3,1]
 * Output: [0,2,3]
 * Explanation: Building 1 (0-indexed) does not have an ocean view because building 2 is taller.
 *
 * <strong class="example">Example 2:
 *
 * Input: heights = [4,3,2,1]
 * Output: [0,1,2,3]
 * Explanation: All the buildings have an ocean view.
 *
 * <strong class="example">Example 3:
 *
 * Input: heights = [1,3,2,4]
 * Output: [3]
 * Explanation: Only building 3 has an ocean view.
 *
 *  
 * Constraints:
 *
 *     1 <= heights.length <= 10^5
 *     1 <= heights[i] <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/buildings-with-an-ocean-view/
// discuss: https://leetcode.com/problems/buildings-with-an-ocean-view/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // pub fn find_buildings(heights: Vec<i32>) -> Vec<i32> {
    //     let mut set = vec![];
    //     for i in 0..heights.len() {
    //         while !set.is_empty() && heights[*set.last().unwrap() as usize] <= heights[i] {
    //             set.pop();
    //         }
    //         set.push(i as i32)
    //     }

    //     set
    // }

    pub fn find_buildings(heights: Vec<i32>) -> Vec<i32> {
        let mut prev = 0;
        let mut buildings = vec![];

        for (pos, building) in heights.iter().rev().enumerate() {
            if *building > prev {
                buildings.push((heights.len() - 1 - pos) as i32);
                prev = *building;
            }
        }
        buildings.into_iter().rev().collect::<Vec<_>>()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1762_1() {
        assert_eq!(Solution::find_buildings(vec![4, 2, 3, 1]), vec![0, 2, 3]);
    }

    #[test]
    fn test_1762_2() {
        assert_eq!(Solution::find_buildings(vec![4, 3, 2, 1]), vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_1762_3() {
        assert_eq!(Solution::find_buildings(vec![1, 3, 2, 4]), vec![3]);
    }
}
