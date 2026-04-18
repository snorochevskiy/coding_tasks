// 3. Longest Substring Without Repeating Characters
// Given a string s, find the length of the longest without duplicate characters.
//
// Example 1:
//   Input: s = "abcabcbb"
//   Output: 3
//   Explanation: The answer is "abc", with the length of 3. Note that "bca" and "cab" are also correct answers.
//
// Example 2:
//   Input: s = "bbbbb"
//   Output: 1
//   Explanation: The answer is "b", with the length of 1.
//
// Example 3:
//   Input: s = "pwwkew"
//   Output: 3
//   Explanation: The answer is "wke", with the length of 3.
// Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.
//
// Constraints:
//   0 <= s.length <= 5 * 104
//   s consists of English letters, digits, symbols and spaces.

fn main() {
    assert!(length_of_longest_substring("abcabcbb".to_string()), 3);
}

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut inds: [i32; 256] = [-1; 256];
    let mut left = 0;
    let mut current_length = 0;
    let mut max_length = 0;

    for (i, c) in s.chars().enumerate() {
        let code = c as usize;
        assert!(code < 266);

        if inds[code] != -1 {
            if inds[code] < left {
                inds[code] = -1;
            } else {
                left = inds[code] + 1;
                current_length = i as i32 - left;
            }
        }
        inds[code] = i as i32;
        current_length += 1;
        if max_length < current_length {
            max_length = current_length;
        }
    }

    max_length
}
