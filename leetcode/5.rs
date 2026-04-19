// 5. Longest Palindromic Substring
// 
// Given a string s, return the longest palindromic string in s.
//
// Example 1:
//   Input: s = "babad"
//   Output: "bab"
//   Explanation: "aba" is also a valid answer.
//
// Example 2:
//   Input: s = "cbbd"
//   Output: "bb"
//
// Constraints:
//   1 <= s.length <= 1000
//   s consist of only digits and English letters.

fn main() {
    println!("{}", longest_palindrome("bb".to_string()));
}

pub fn longest_palindrome(s: String) -> String {
    let chars = s.as_bytes();

    let mut longest: &[u8] = &[];

    for i in 0 .. chars.len() {
        let current = find_polyndrome(chars, i,i);
        if longest.len() < current.len() {
            longest = current;
        }
        if i + 1 < chars.len() && chars[i] == chars[i+1] {
            let current = find_polyndrome(chars, i, i + 1);
            if longest.len() < current.len() {
                longest = current;
            }
        }
    }

    String::from_utf8(longest.to_vec()).unwrap()
}

#[inline]
fn find_polyndrome(chars: &[u8], mut l: usize, mut r: usize) -> &[u8] {
    let mut current;
    loop {
        current = &chars[l .. r + 1];
        if l == 0 || r == chars.len() - 1 {
            break;
        }
        l -= 1;
        r += 1;
        if chars[l] != chars[r] {
            break;
        }
    }
    current
}

