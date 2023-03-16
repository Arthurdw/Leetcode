pub struct Solution;

impl Solution {
    pub fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
        let mut last_missing = 0;
        let mut missing_count = 0;

        for &num in &arr {
            missing_count += num - last_missing - 1;

            if missing_count >= k {
                return last_missing + k - (missing_count - (num - last_missing - 1));
            }

            last_missing = num;
        }

        return last_missing + k - missing_count;
    }
}

#[test]
fn test() {
    let arr = vec![2, 3, 4, 7, 11];
    let k = 5;
    let result = Solution::find_kth_positive(arr, k);
    assert_eq!(result, 9);
}