struct Solution;

impl Solution {
    pub fn using_buildin(haystack: String, needle: String) -> i32 {
        let matches = haystack.match_indices(&needle);

        for (index, _) in matches {
            return index as i32;
        }

        -1
    }

    pub fn str_str(haystack: String, needle: String) -> i32 {
        let mut current_index = 0;
        let mut needle_index = 0;

        if needle.len() == 0 || haystack == needle {
            return 0;
        } else if haystack.len() == 0 || needle.len() > haystack.len() {
            return -1;
        }

        while current_index < haystack.len() {
            if haystack.chars().nth(current_index) == needle.chars().nth(needle_index) {
                needle_index += 1;
            } else {
                current_index -= needle_index;
                needle_index = 0;
            }

            if needle_index == needle.len() {
                return (current_index - needle_index + 1) as i32;
            }

            current_index += 1;
        }

        -1
    }
}

#[test]
fn test() {
    let haystack = "hello".to_string();
    let needle = "ll".to_string();
    let result = Solution::str_str(haystack.clone(), needle.clone());
    let result2 = Solution::using_buildin(haystack, needle);
    assert_eq!(result, 2);
    assert_eq!(result2, 2);
}

#[test]
fn test2() {
    let haystack = "aaaaa".to_string();
    let needle = "bba".to_string();
    let result = Solution::str_str(haystack.clone(), needle.clone());
    let result2 = Solution::using_buildin(haystack, needle);
    assert_eq!(result, -1);
    assert_eq!(result2, -1);
}

#[test]
fn test3() {
    let haystack = "".to_string();
    let needle = "".to_string();
    let result = Solution::str_str(haystack.clone(), needle.clone());
    let result2 = Solution::using_buildin(haystack, needle);
    assert_eq!(result, 0);
    assert_eq!(result2, 0);
}

#[test]
fn test4() {
    let haystack = "a".to_string();
    let needle = "a".to_string();
    let result = Solution::str_str(haystack.clone(), needle.clone());
    let result2 = Solution::using_buildin(haystack, needle);
    assert_eq!(result, 0);
    assert_eq!(result2, 0);
}

#[test]
fn test5() {
    let haystack = "mississippi".to_string();
    let needle = "issip".to_string();
    let result = Solution::str_str(haystack.clone(), needle.clone());
    let result2 = Solution::using_buildin(haystack, needle);
    assert_eq!(result, 4);
    assert_eq!(result2, 4);
}

#[test]
fn test6() {
    let haystack = "issi".to_string();
    let needle = "issipi".to_string();
    let result = Solution::str_str(haystack.clone(), needle.clone());
    let result2 = Solution::using_buildin(haystack, needle);
    assert_eq!(result, -1);
    assert_eq!(result2, -1);
}