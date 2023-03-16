struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if 0 <= x && x < 10 {
            return true;
        }

        if x < 0 || x % 10 == 0 {
            return false;
        }

        let radix = 10;
        let mut n = x;
        let mut reversed = 0;

        while n != 0 {
            reversed = reversed * radix + n % radix;
            n /= radix;
        }

        x == reversed
    }
}

#[test]
fn positive() {
    assert_eq!(Solution::is_palindrome(121), true);
}

#[test]
fn negative() {
    assert_eq!(Solution::is_palindrome(-121), false);
    assert_eq!(Solution::is_palindrome(-101), false);
}

#[test]
fn zero() {
    assert_eq!(Solution::is_palindrome(0), true);
}

#[test]
fn with_zero() {
    assert_eq!(Solution::is_palindrome(1000021), false);
    assert_eq!(Solution::is_palindrome(10), false);
    assert_eq!(Solution::is_palindrome(100010001), true);
}