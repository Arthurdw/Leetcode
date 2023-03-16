struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut last_idx: i32 = 0;

        for i in 1..nums.len() {
            if nums[i] != nums[last_idx as usize] {
                last_idx += 1;
                nums[last_idx as usize] = nums[i];
            }
        }

        last_idx + 1
    }
}

#[test]
fn test() {
    let mut nums = vec![1, 1, 2];
    let len = Solution::remove_duplicates(&mut nums);
    assert_eq!(len, 2);
    assert_eq!(nums, vec![1, 2, 2]);
}

#[test]
fn test2() {
    let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    let len = Solution::remove_duplicates(&mut nums);
    assert_eq!(len, 5);
    assert_eq!(nums, vec![0, 1, 2, 3, 4, 2, 2, 3, 3, 4]);
}