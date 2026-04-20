// 9. Palindrome Number
//
// Given an integer x, return true if x is a palindrome, and false otherwise.
//
// Example 1:
//   Input: x = 121
//   Output: true
// Explanation: 121 reads as 121 from left to right and from right to left.
//
// Example 2:
//   Input: x = -121
//   Output: false
// Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.
//
// Example 3:
//   Input: x = 10
//   Output: false
// Explanation: Reads 01 from right to left. Therefore it is not a palindrome.
//
// Constraints:
//
//     -2^31 <= x <= 2^31 - 1
//
// Follow up: Could you solve it without converting the integer to a string?

fn main() {
    assert_eq!(true, is_palindrome(121));
    assert_eq!(false, is_palindrome(-121));
    assert_eq!(false, is_palindrome(10));
}

pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }
    if x < 10 {
        return true;
    }

    let mut x_left = x;
    let mut divider = 1000000000;
    let mut left_digit = loop {
        let res = x / divider;
        if res != 0 {
            x_left = x % divider;
            divider /= 10;
            break res;
        }
        divider /= 10;
    };

    let mut right_digit = x % 10;
    let mut x_right = x / 10;

    while x_right != 0 {
        if left_digit != right_digit {
            return false;
        }
        right_digit = x_right % 10;
        x_right = x_right / 10;

        left_digit = x_left / divider;
        x_left = x % divider;
        divider /= 10;
    }

    true
}

