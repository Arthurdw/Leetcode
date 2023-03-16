struct Solution;

impl Solution {
    fn char_matches(expected: u8, chars: &Vec<Box<&[u8]>>, index: usize) -> bool {
        for c in chars.iter() {
            if c.len() <= index {
                return false;
            }

            let val = c[index];

            if val != expected {
                return false;
            }
        }

        true
    }

    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let chars: Vec<Box<&[u8]>> = strs.iter().map(|s| Box::new(s.as_bytes())).collect();

        let mut common_prefix = String::new();
        let mut current_idx = 0;

        loop {
            let current_char = if chars[0].len() > current_idx {
                chars[0][current_idx]
            } else {
                break;
            };

            if !Solution::char_matches(current_char, &chars, current_idx) {
                break;
            }

            common_prefix.push(current_char as char);
            current_idx += 1;
        }

        common_prefix
    }
}

#[test]
fn match_first() {
    let strs = vec![
        "flower".to_string(),
        "flow".to_string(),
        "flight".to_string(),
    ];
    let result = Solution::longest_common_prefix(strs);
    assert_eq!(result, "fl".to_string());
}

#[test]
fn no_match() {
    let strs = vec!["dog".to_string(), "racer".to_string(), "car".to_string()];
    let result = Solution::longest_common_prefix(strs);
    assert_eq!(result, "".to_string());
}
