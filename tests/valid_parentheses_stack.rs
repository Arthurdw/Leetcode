// This is a quite improved version of ./valid_parentheses.rs
// Using the ascii table characteristics and using a Stack (from the Vector implementation)

struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::with_capacity(s.len());

        for c in s.bytes() {
            match c {
                sel @ (b'(' | b'[' | b'{') => stack.push(sel),
                sel @ (b')' | b']' | b'}') => {
                    if stack
                        .pop()
                        .map_or(true, |last| last + 1 != sel && last + 2 != sel)
                    {
                        return false;
                    }
                }
                _ => {} // Ignore all other characters
            }
        }

        stack.is_empty()
    }
}

#[test]
fn test() {
    assert!(Solution::is_valid("()".to_string()));
    assert!(Solution::is_valid("()[]{}".to_string()));
    assert!(!Solution::is_valid("(]".to_string()));
    assert!(!Solution::is_valid("([)]".to_string()));
    assert!(Solution::is_valid("{[]}".to_string()));
}

