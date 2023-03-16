struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut destruct = Vec::new();

        for c in s.chars() {
            match c {
                sel @ ('(' | '{' | '[') => destruct.push(match sel {
                    '(' => ')',
                    '{' => '}',
                    '[' => ']',
                    _ => panic!("This should not be here, but is required because of a bugg within rust (https://github.com/rust-lang/rust/issues/108932)")
                }),
                sel @ (')' | '}' | ']') if destruct.len() > 0 && *destruct.last().unwrap() == sel => { destruct.pop().unwrap(); },
                _ => return false
            }
        }

        destruct.len() == 0
    }
}

#[test]
fn test() {
    assert_eq!(Solution::is_valid("()".to_string()), true);
    assert_eq!(Solution::is_valid("()[]{}".to_string()), true);
    assert_eq!(Solution::is_valid("(]".to_string()), false);
    assert_eq!(Solution::is_valid("([)]".to_string()), false);
    assert_eq!(Solution::is_valid("{[]}".to_string()), true);
}