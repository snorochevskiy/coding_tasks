// 41. First Missing Positive
//
// Given an unsorted integer array nums. Return the smallest positive integer that is not present in nums.
// You must implement an algorithm that runs in O(n) time and uses O(1) auxiliary space.
//
// Example 1:
//   Input: nums = [1,2,0]
//   Output: 3
// Explanation: The numbers in the range [1,2] are all in the array.
//
// Example 2:
//   Input: nums = [3,4,-1,1]
//   Output: 2
// Explanation: 1 is in the array but 2 is missing.
//
// Example 3:
//   Input: nums = [7,8,9,11,12]
//   Output: 1
// Explanation: The smallest positive integer 1 is missing.
//
// Constraints:
//   1 <= nums.length <= 10^5
//   -2^31 <= nums[i] <= 2^31 - 1

fn main() {
    assert_eq!(3, Solution::first_missing_positive(vec![1, 2, 0]));
    assert_eq!(2, Solution::first_missing_positive(vec![3,4,-1,1]));
    assert_eq!(2, Solution::first_missing_positive(vec![1]));
    assert_eq!(1, Solution::first_missing_positive(vec![2147483647]));
}

struct Solution;

impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        for i in  0 .. nums.len() {
            if nums[i] > 0 && nums[i] <= nums.len() as i32 {
                let mut dst_ind = nums[i] - 1;
                loop {
                    if nums[dst_ind as usize] > 0 && nums[dst_ind as usize] <= nums.len() as i32 && nums[dst_ind as usize] != dst_ind + 1 {
                        let tmp = nums[dst_ind as usize];
                        nums[dst_ind as usize] = dst_ind + 1;
                        dst_ind = tmp - 1;
                    } else {
                        nums[dst_ind as usize] = dst_ind + 1;
                        break;
                    }
                }
            }
        }

        for i in 0 .. nums.len() {
            if nums[i] != (i + 1) as i32 {
                return (i + 1) as i32;
            }
        }

        nums.len() as i32 + 1
    }
}
