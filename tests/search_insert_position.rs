struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        if nums.last().unwrap() < &target {
            return nums.len() as i32;
        }

        nums.iter()
            .enumerate()
            .find(|(_, num)| **num == target || **num > target)
            .map(|(i, _)| i as i32)
            .unwrap_or(-1)
    }
}

#[test]
fn test() {
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
}