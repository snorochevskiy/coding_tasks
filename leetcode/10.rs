// 10. Regular Expression Matching
//
// Given an input string s and a pattern p, implement regular expression matching with support for '.' and '*' where:
//     '.' Matches any single character.​​​​
//     '*' Matches zero or more of the preceding element.
// Return a boolean indicating whether the matching covers the entire input string (not partial).
//
// Example 1:
//   Input: s = "aa", p = "a"
//   Output: false
// Explanation: "a" does not match the entire string "aa".
//
// Example 2:
//   Input: s = "aa", p = "a*"
//   Output: true
// Explanation: '*' means zero or more of the preceding element, 'a'. Therefore, by repeating 'a' once, it becomes "aa".
//
// Example 3:
//   Input: s = "ab", p = ".*"
//   Output: true
// Explanation: ".*" means "zero or more (*) of any character (.)".
//
// Constraints:
//   1 <= s.length <= 20
//   1 <= p.length <= 20
//   s contains only lowercase English letters.
//   p contains only lowercase English letters, '.', and '*'.
// It is guaranteed for each appearance of the character '*', there will be a previous valid character to match.

fn main() {
    assert_eq!(true, is_match("aab".to_string(), "c*a*b".to_string()));
    assert_eq!(false, is_match("ab".to_string(), ".*c".to_string()));
    assert_eq!(true, is_match("ab".to_string(), ".*".to_string()));
    assert_eq!(true, is_match("aaa".to_string(), "a*a".to_string()));
    assert_eq!(false, is_match("aaba".to_string(), "ab*a*c*a".to_string()));
    assert_eq!(true, is_match("aaca".to_string(), "ab*a*c*a".to_string()));
}

#[derive(Clone, Copy)]
struct Pat {
    c: char,
    repeated: bool,
}

use std::fmt::Write;
impl std::fmt::Debug for Pat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.c)?;
        if self.repeated {
            f.write_char('*')?;
        }
        Ok(())
    }
}

fn parse_re(re: &str) -> Vec<Pat> {
    let mut result: Vec<Pat> = Vec::new();
    for c in re.chars() {
        if c == '*' {
            result.last_mut().unwrap().repeated = true;
        } else {
            result.push(Pat { c, repeated: false });
        }
    }
    result
}

pub fn is_match(s: String, p: String) -> bool {
    let chars = s.as_bytes();
    let patterns = parse_re(&p);

    let mut char_ind = 0;
    let mut pat_ind = 0;

    let mut searches: Vec<(usize, usize)> = Vec::new();

    let mut is_went_back = false;

    loop {
        let current_input = chars.get(char_ind).map(|c|*c as char);
        let current_pat = patterns.get(pat_ind);

        if is_went_back {
            char_ind += 1;
            is_went_back = false;
            continue;
        }

        match (current_input, current_pat) {
            (Some(_), Some(Pat { c: '.', repeated: true })) => {
                searches.push((char_ind, pat_ind));
                pat_ind += 1;
            }
            (Some(_), Some(Pat { c: '.', repeated: false })) => {
                char_ind += 1;
                pat_ind += 1;
            }
            (Some(input_c), Some(Pat { c, repeated: true })) => {
                if input_c == *c {
                    searches.push((char_ind, pat_ind));
                    pat_ind += 1;
                } else {
                    pat_ind += 1;
                }
            }
            (Some(input_c), Some(Pat { c, repeated: false })) => {
                if input_c != *c {
                    if searches.is_empty() {
                        return false;
                    } else {
                        (char_ind, pat_ind) = searches.pop().unwrap();
                        is_went_back = true;
                    }
                } else {
                    char_ind += 1;
                    pat_ind += 1;
                }
            }
            (None, Some(Pat { c: _, repeated: false })) => {
                    if searches.is_empty() {
                        return false;
                    } else {
                        (char_ind, pat_ind) = searches.pop().unwrap();
                        is_went_back = true;
                    }
            }
            (None, Some(Pat { c: _, repeated: true })) => {
                pat_ind += 1;
            }
            (Some(_), None) => {
                    if searches.is_empty() {
                        return false;
                    } else {
                        (char_ind, pat_ind) = searches.pop().unwrap();
                        is_went_back = true;
                    }
            }
            (None, None) => {
                return true;
            }
            _ => unreachable!(),
        }
    }
}
