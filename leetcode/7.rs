// 7. Reverse Integer
//
// Given a signed 32-bit integer x, return x with its digits reversed.
// If reversing x causes the value to go outside the signed 32-bit integer range [-231, 231 - 1], then return 0.
//
// Assume the environment does not allow you to store 64-bit integers (signed or unsigned).
//
// Example 1:
//   Input: x = 123
//   Output: 321
//
// Example 2:
//   Input: x = -123
//   Output: -321
//
// Example 3:
//  Input: x = 120
//  Output: 21
//
// Constraints:
//   -231 <= x <= 231 - 1

fn main() {
    assert_eq!(321, reverse(123));
    assert_eq!(-321, reverse(-123));
}

pub fn reverse(x: i32) -> i32 {
    let mut input = x;
    let mut result = 0i32;
    while input != 0 {
        let last_digit = input % 10;
        let (temp_res, overflow) = result.overflowing_mul(10);
        if overflow {
            return 0;
        }
        let (temp_res, overflow) = temp_res.overflowing_add(last_digit);
        if overflow {
            return 0;
        }
        result = temp_res;
        input = input / 10;
    }

    result
}
