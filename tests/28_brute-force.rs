// We want to find the first occurance of a needle within a haystack.
// We will do this using the brute force algorithm.
//
// This results in being O(h*n) where h is the length of the haystack
// and n is the length of the needle.

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

        'outer: for haystack_i in 0..(haystack.len() - needle.len() + 1) {
            for needle_i in 0..needle.len() {
                if haystack_bytes[haystack_i + needle_i] != needle_bytes[needle_i] {
                    continue 'outer;
                }
            }

            return haystack_i as i32;
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
