// 12. Integer to Roman
//
// Seven different symbols represent Roman numerals with the following values:
// 1=I, 5=V, 10=X, 50=L, 100=C, 500=D, 1000=M
//
// Roman numerals are formed by appending the conversions of decimal place values from highest to lowest.
// Converting a decimal place value into a Roman numeral has the following rules:
// * If the value does not start with 4 or 9, select the symbol of the maximal value that can be subtracted from the input,
//   append that symbol to the result, subtract its value, and convert the remainder to a Roman numeral.
// * If the value starts with 4 or 9 use the subtractive form representing one symbol subtracted from the following symbol,
//   for example, 4 is 1 (I) less than 5 (V): IV and 9 is 1 (I) less than 10 (X): IX.
//   Only the following subtractive forms are used: 4 (IV), 9 (IX), 40 (XL), 90 (XC), 400 (CD) and 900 (CM).
// * Only powers of 10 (I, X, C, M) can be appended consecutively at most 3 times to represent multiples of 10.
//   You cannot append 5 (V), 50 (L), or 500 (D) multiple times.
//   If you need to append a symbol 4 times use the subtractive form.
//
// Given an integer, convert it to a Roman numeral.
//
// Example 1:
//   Input: num = 3749
//   Output: "MMMDCCXLIX"
// Explanation:
// 3000 = MMM as 1000 (M) + 1000 (M) + 1000 (M)
//  700 = DCC as 500 (D) + 100 (C) + 100 (C)
//   40 = XL as 10 (X) less of 50 (L)
//    9 = IX as 1 (I) less of 10 (X)
// Note: 49 is not 1 (I) less of 50 (L) because the conversion is based on decimal places
//
// Example 2:
//   Input: num = 58
//   Output: "LVIII"
// Explanation:
// 50 = L
//  8 = VIII
//
// Example 3:
//   Input: num = 1994
//   Output: "MCMXCIV"
// Explanation:
// 1000 = M
//  900 = CM
//   90 = XC
//    4 = IV
//
// Constraints:
//   1 <= num <= 3999

fn main() {
    assert_eq!("MMMDCCXLIX".to_string(), int_to_roman(3749));
    assert_eq!("CCCLXV", int_to_roman(365));
    assert_eq!("LVIII", int_to_roman(58)); 
}

pub fn int_to_roman(num: i32) -> String {
    let mut number = num;
    let mut digits = Vec::new();
    while number != 0 {
        digits.push(number % 10);
        number /= 10;
    }

    let mut result = String::new();

    for (i, d) in digits.iter().rev().enumerate() {
        add_roman_digit(&mut result, *d, &ROM_RANKS[digits.len() - i - 1]);
    }

    result
}

const ROM_RANKS: &[(char, char, char)] = &[
    ('I', 'V', 'X'), ('X', 'L', 'C'), ('C', 'D', 'M'), ('M', ' ', ' ')
];

fn add_roman_digit(result: &mut String, digit: i32, romans: &(char, char, char)) {
    match digit {
        0 => (),
        1 => {
            result.push(romans.0);
        },
        2 => {
            result.push(romans.0);
            result.push(romans.0);
        }
        3 => {
            result.push(romans.0);
            result.push(romans.0);
            result.push(romans.0);
        }
        4 => {
            result.push(romans.0);
            result.push(romans.1);
        }
        5 => {
            result.push(romans.1);
        }
        6 => {
            result.push(romans.1);
            result.push(romans.0);
        }
        7 => {
            result.push(romans.1);
            result.push(romans.0);
            result.push(romans.0);
        }
        8 => {
            result.push(romans.1);
            result.push(romans.0);
            result.push(romans.0);
            result.push(romans.0);
        }
        9 => {
            result.push(romans.0);
            result.push(romans.2);
        }
        _ => unreachable!(),
    }
}
