//11. Container With Most Water
//
// You are given an integer array height of length n.
// There are n vertical lines drawn such that the two endpoints of the i-th line are (i, 0) and (i, height[i]).
//
// Find two lines that together with the x-axis form a container, such that the container contains the most water.
//
// Return the maximum amount of water a container can store.
// 
// Notice that you may not slant the container.
//
// Example 1:
//   Input: height = [1,8,6,2,5,4,8,3,7]
//   Output: 49
// Explanation: The above vertical lines are represented by array [1,8,6,2,5,4,8,3,7].
//              In this case, the max area of water (blue section) the container can contain is 49.
// Example 2:
//   Input: height = [1,1]
//   Output: 1
//
// Constraints:
//   n == height.length
//   2 <= n <= 105
//   0 <= height[i] <= 104


fn main() {
    assert_eq!(49, max_area(vec![1,8,6,2,5,4,8,3,7]));
    assert_eq!(1, max_area(vec![1,1]));
}

pub fn max_area(height: Vec<i32>) -> i32 {
    let mut l = 0;
    let mut r = height.len() - 1;

    let mut max_area = (height[l].min(height[r]) * (r - l) as i32, [l, r]);

    while l != r {
        if height[l] < height[r] {
            l += 1;
        } else {
            r -= 1;
        }
        let cur_area = height[l].min(height[r]) * (r - l) as i32;
        if max_area.0 < cur_area {
            max_area = (cur_area, [l, r]);
        }
    }

    max_area.0
}
