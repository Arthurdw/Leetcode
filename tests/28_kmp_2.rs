// Same as ./28_kmp.rs but created as repetition

struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.is_empty() {
            return 0;
        }

        let needle = needle.as_bytes();
        let haystack = haystack.as_bytes();

        let mut lps = vec![0; needle.len()];
        let mut i = 1;
        let mut prev_lps = 0;

        while i < needle.len() {
            if needle[i] == needle[prev_lps] {
                lps[i] = prev_lps + 1;
                prev_lps += 1;
                i += 1;
            } else if prev_lps == 0 {
                lps[i] = 0;
                i += 1;
            } else {
                prev_lps = lps[prev_lps - 1];
            }
        }

        let mut i = 0;
        let mut j = 0;

        while i < haystack.len() {
            if haystack[i] == needle[j] {
                i += 1;
                j += 1;
            } else if j == 0 {
                i += 1;
            } else {
                j = lps[j - 1];
            }

            if j == needle.len() {
                return (i - needle.len()) as i32;
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
        assert_eq!(
            Solution::str_str("aaacaaabaaacaaaa".to_string(), "aaacaaaa".to_string()),
            8
        );
        assert_eq!(Solution::str_str("hello".to_string(), "ll".to_string()), 2);
        assert_eq!(Solution::str_str("hello".to_string(), "lo".to_string()), 3);
        assert_eq!(
            Solution::str_str("ababcaababcaabc".to_string(), "ababcaabc".to_string()),
            6
        );
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
