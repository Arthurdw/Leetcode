/// A solution to the Roman to Integer problem on leetcode.
/// This solution is a bit more memory intensive (because if the use of enums),
/// but is really well partitioned and should be easy to expand further upon.
#[derive(PartialEq)]
enum Roman {
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

impl Roman {
    pub fn value(&self) -> u16 {
        match &self {
            Roman::I => 1,
            Roman::V => 5,
            Roman::X => 10,
            Roman::L => 50,
            Roman::C => 100,
            Roman::D => 500,
            Roman::M => 1000
        }
    }


    pub fn from(value: char) -> Roman {
        match value {
            'I' => Roman::I,
            'V' => Roman::V,
            'X' => Roman::X,
            'L' => Roman::L,
            'C' => Roman::C,
            'D' => Roman::D,
            'M' => Roman::M,
            _ => panic!("Invalid Roman Numeral")
        }
    }
}

fn match_next(src: &Roman, next: &Roman, opt1: Roman, opt2: Roman) -> (u16, usize) {
    match next {
        x if x == &opt1 => (opt1.value() - src.value(), 2),
        x if x == &opt2 => (opt2.value() - src.value(), 2),
        _ => (src.value(), 1)
    }
}

fn parse_next(full: &str) -> (u16, &str) {
    let char = full.chars().next().unwrap();
    let next = full.chars().nth(1);
    let roman = Roman::from(char);

    if next.is_none() {
        return (roman.value(), "");
    }

    let next_roman = Roman::from(next.unwrap());

    let next = |o1: Roman, o2: Roman| -> (u16, &str) {
        let (value, skip) = match_next(&roman, &next_roman, o1, o2);
        (value, &full[skip..])
    };

    match roman {
        Roman::I => next(Roman::V, Roman::X),
        Roman::X => next(Roman::L, Roman::C),
        Roman::C => next(Roman::D, Roman::M),
        _ => (roman.value(), &full[1..])
    }
}

fn parse_roman(s: &str) -> u16 {
    let mut total = 0;
    let mut remaining = s;

    while remaining.len() > 0 {
        let (value, next) = parse_next(remaining);
        total += value;
        remaining = next;
    }

    total
}

struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        parse_roman(&s) as i32
    }
}

#[test]
fn test_roman_to_int() {
    assert_eq!(3, Solution::roman_to_int("III".to_string()));
    assert_eq!(4, Solution::roman_to_int("IV".to_string()));
    assert_eq!(9, Solution::roman_to_int("IX".to_string()));
    assert_eq!(58, Solution::roman_to_int("LVIII".to_string()));
    assert_eq!(1994, Solution::roman_to_int("MCMXCIV".to_string()));
}