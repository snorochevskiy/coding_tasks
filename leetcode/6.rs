// 6. Zigzag Conversion
//
// The string "PAYPALISHIRING" is written in a zigzag pattern on a given number of rows like this: (you may want to display this pattern in a fixed font for better legibility)
//
// P   A   H   N
// A P L S I I G
// Y   I   R
//
// And then read line by line: "PAHNAPLSIIGYIR"
//
// Write the code that will take a string and make this conversion given a number of rows:
//
// Example 1:
//   Input: s = "PAYPALISHIRING", numRows = 3
//   Output: "PAHNAPLSIIGYIR"

// Example 2:
//   Input: s = "PAYPALISHIRING", numRows = 4
//   Output: "PINALSIGYAHRPI"
// Explanation:
// P     I    N
// A   L S  I G
// Y A   H R
// P     I
//
// Example 3:
//   Input: s = "A", numRows = 1
//   Output: "A"
//
// Constraints:
//   1 <= s.length <= 1000
//   s consists of English letters (lower-case and upper-case), ',' and '.'.
//   1 <= numRows <= 1000


fn main() {
    assert_eq!(convert("PAYPALISHIRING".to_string(), 3), "PAHNAPLSIIGYIR".to_string());
    assert_eq!(convert("PAYPALISHIRING".to_string(), 4), "PINALSIGYAHRPI".to_string());
}

pub fn convert(s: String, num_rows: i32) -> String {

    let mut rows: Vec<Vec<u8>> = (0..num_rows).map(|_| Vec::new())
        .collect();

    let mut indexes = Vec::new();
    for i in 0 .. num_rows as usize {
        indexes.push(i);
    }
    for i in (1 .. (num_rows as usize).saturating_sub(1)).rev() {
        indexes.push(i);
    }

    for (&c, &row) in s.as_bytes().iter().zip(indexes.iter().cycle()) {
        rows[row].push(c);
    }

    let mut result = String::new();
    for row in rows {
        result.push_str(str::from_utf8(&row).unwrap());
    }

    result
}

