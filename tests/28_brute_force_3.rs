// Same as ./28_brute-force.rs but allow string comparison.

struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.is_empty() {
            return 0;
        }

        for i in 0..(haystack.len() - needle.len() + 1) {
            if haystack[i..i + needle.len()] == needle {
                return i as i32;
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
}
