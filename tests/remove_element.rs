struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut last_idx: i32 = -1;

        for i in 0..nums.len() {
            if nums[i] != val {
                last_idx += 1;
                nums[last_idx as usize] = nums[i];
            }
        }

        last_idx + 1
    }
}

#[test]
fn test() {
    let mut nums = vec![3, 2, 2, 3];
    let len = Solution::remove_element(&mut nums, 3);
    assert_eq!(len, 2);
    assert_eq!(nums, vec![2, 2, 2, 3]);
}