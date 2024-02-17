#![allow(unused_imports)]
use rand::Rng;
/**
 * [528] Random Pick with Weight
 *
 * You are given a 0-indexed array of positive integers w where w[i] describes the weight of the i^th index.
 * You need to implement the function pickIndex(), which randomly picks an index in the range [0, w.length - 1] (inclusive) and returns it. The probability of picking an index i is w[i] / sum(w).
 *
 * For example, if w = [1, 3], the probability of picking index 0 is 1 / (1 + 3) = 0.25 (i.e., 25%), and the probability of picking index 1 is 3 / (1 + 3) = 0.75 (i.e., 75%).
 *
 *  
 * <strong class="example">Example 1:
 *
 * Input
 * ["Solution","pickIndex"]
 * [[[1]],[]]
 * Output
 * [null,0]
 * Explanation
 * Solution solution = new Solution([1]);
 * solution.pickIndex(); // return 0. The only option is to return 0 since there is only one element in w.
 *
 * <strong class="example">Example 2:
 *
 * Input
 * ["Solution","pickIndex","pickIndex","pickIndex","pickIndex","pickIndex"]
 * [[[1,3]],[],[],[],[],[]]
 * Output
 * [null,1,1,1,1,0]
 * Explanation
 * Solution solution = new Solution([1, 3]);
 * solution.pickIndex(); // return 1. It is returning the second element (index = 1) that has a probability of 3/4.
 * solution.pickIndex(); // return 1
 * solution.pickIndex(); // return 1
 * solution.pickIndex(); // return 1
 * solution.pickIndex(); // return 0. It is returning the first element (index = 0) that has a probability of 1/4.
 * Since this is a randomization problem, multiple answers are allowed.
 * All of the following outputs can be considered correct:
 * [null,1,1,1,1,0]
 * [null,1,1,1,1,1]
 * [null,1,1,1,0,0]
 * [null,1,1,1,0,1]
 * [null,1,0,1,0,0]
 * ......
 * and so on.
 *
 *  
 * Constraints:
 *
 *  1 <= w.length <= 10^4
 *  1 <= w[i] <= 10^5
 *  pickIndex will be called at most 10^4 times.
 *
 */
// pub struct Solution {}

// problem: https://leetcode.com/problems/random-pick-with-weight/
// discuss: https://leetcode.com/problems/random-pick-with-weight/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// SLOWEST SOLUTION
// struct Solution {
//     prefix_sums: Vec<i32>,
// }

// impl Solution {
//     fn new(w: Vec<i32>) -> Self {
//         let prefix_sums = w
//             .iter()
//             .scan(0, |acc, &x| {
//                 *acc += x;
//                 Some(*acc)
//             })
//             .collect();

//         Self { prefix_sums }
//     }

//     fn pick_index(&mut self) -> i32 {
//         // For some reason, the LC compiler wants gen_range to be
//         // called as gen_range(f32, f32) rather than as a slice?
//         // let rand_num = rand::thread_rng().gen_range(0.0, 1.0);

//         let rand_num = rand::thread_rng().gen_range(0.0..1.0);
//         let target = rand_num * *self.prefix_sums.last().unwrap() as f32;
//         let index = self
//             .prefix_sums
//             .iter()
//             .position(|&x| x as f32 > target)
//             .unwrap();

//         index as i32
//     }
// }

// MEDIUM SOLUTION
// struct Solution {
//     btm: BTreeMap<i32, usize>,
// }
//
// impl Solution {
//     fn new(w: Vec<i32>) -> Self {
//         let mut btm: BTreeMap<i32, usize> = BTreeMap::new();
//         let mut n = 0;

//         // Calculate the cumulative sum of weights and store it in a BTreeMap.
//         // The key represents the cumulative sum and the value represents the index.
//         for weight in w.iter() {
//             n += *weight;
//             btm.insert(n, btm.len());
//         }

//         Self { btm, n }
//     }

//     /// Picks an index randomly based on the weights.
//     fn pick_index(&self) -> i32 {
//         // Generate a random number between 0 and self.n (exclusive).
//         let mut rng = rand::thread_rng();
//         let m: i32 = rng.gen_range(0, self.n);

//         // Find the first key in the BTreeMap that is greater than m.
//         // The corresponding value represents the picked index.
//         if let Some(next) = self.btm.range(m + 1..).next() {
//             *next.1 as i32
//         } else {
//             -1
//         }
//     }
// }

// FASTEST SOLUTION
struct Solution {
    wsum: Vec<i32>,
    seed: i64,
    last: i32,
}

impl Solution {
    fn new(w: Vec<i32>) -> Self {
        let wsum: Vec<i32> = w
            .iter()
            .scan(0, |sum, x| {
                *sum += x;
                Some(*sum)
            })
            .collect();

        let last = *wsum.last().unwrap();
        Self {
            wsum,
            seed: 1,
            last,
        }
    }

    fn pick_index(&mut self) -> i32 {
        self.seed = (self.seed * 1_000_000_007 + 257) % 1_000_000_009;
        match self.wsum.binary_search(&(self.seed as i32 % self.last)) {
            Ok(i) => i as i32 + 1,
            Err(i) => i as i32,
        }
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(w);
 * let ret_1: i32 = obj.pick_index();
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_528_1() {}
}
