// Same as ./28_brute-force.rs but use less rust specific logic

struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.is_empty() {
            return 0;
        }

        if needle.len() > haystack.len() {
            return -1;
        }

        let haystack_bytes = haystack.as_bytes();
        let needle_bytes = needle.as_bytes();

        for i in 0..(haystack.len() - needle.len() + 1) {
            for j in 0..needle.len() {
                if haystack_bytes[i + j] != needle_bytes[j] {
                    break;
                }

                if j == needle.len() - 1 {
                    return i as i32;
                }
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
