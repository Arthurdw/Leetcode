// Same as ./28_brute_force_3.rs but with the windows iterator

struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.is_empty() {
            return 0;
        }

        for (counter, window) in haystack.as_bytes().windows(needle.len()).enumerate() {
            if window == needle.as_bytes() {
                return counter as i32;
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn needle_present() {
        assert_eq!(Solution::str_str("hello".to_string(), "ll".to_string()), 2);
        assert_eq!(Solution::str_str("hello".to_string(), "lo".to_string()), 3);
    }

    #[test]
    fn needle_not_present() {
        assert_eq!(
            Solution::str_str("hello".to_string(), "lll".to_string()),
            -1
        );
    }

    #[test]
    fn needle_empty() {
        assert_eq!(Solution::str_str("hello".to_string(), "".to_string()), 0);
    }

    #[test]
    fn needle_longer_than_haystack() {
        assert_eq!(
            Solution::str_str("hello".to_string(), "helloo".to_string()),
            -1
        );
    }
}
