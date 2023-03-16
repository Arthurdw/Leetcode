use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hash = HashMap::new();

        for (idx, n) in nums.iter().enumerate() {
            if let Some(other_idx) = hash.get(&(target - n)) {
                return vec![idx as i32, *other_idx]
            }

            hash.insert(n, idx as i32);
        }

        panic!("No match was found!");
    }
}

#[test]
fn test() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result = Solution::two_sum(nums, target);

    if result != vec![0, 1] && result != vec![1, 0] {
        panic!("Expected vec![0, 1] or vec![1, 0], got {:?}", result);
    }
}