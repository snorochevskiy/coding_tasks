// 4. Median of Two Sorted Arrays
// 
// Given two sorted arrays nums1 and nums2 of size m and n respectively, return the median of the two sorted arrays.
//
// The overall run time complexity should be O(log (m+n)).
//
// Example 1:
//   Input: nums1 = [1,3], nums2 = [2]
//   Output: 2.00000
//   Explanation: merged array = [1,2,3] and median is 2.
//
// Example 2:
//   Input: nums1 = [1,2], nums2 = [3,4]
//   Output: 2.50000
//   Explanation: merged array = [1,2,3,4] and median is (2 + 3) / 2 = 2.5.
//
// Constraints:
//     nums1.length == m
//     nums2.length == n
//     0 <= m <= 1000
//     0 <= n <= 1000
//     1 <= m + n <= 2000
//     -106 <= nums1[i], nums2[i] <= 106


use std::iter::Peekable;

fn main() {
    assert_eq!(2.0, find_median_sorted_arrays(vec![1,3], vec![2]));
    assert_eq!(2.5, find_median_sorted_arrays(vec![1,2], vec![3,4]));
}

pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let median = (nums1.len() + nums2.len()) / 2;
    let is_odd = (nums1.len() + nums2.len()) % 2 == 1;

    let mut curr_num = 0;

    let mut it_1 = nums1.iter().peekable();
    let mut it_2 = nums2.iter().peekable();

    #[inline]
    fn get_less<'a>(it_1: &mut Peekable<impl Iterator<Item = &'a i32>>, it_2: &mut Peekable<impl Iterator<Item = &'a i32>>) -> i32 {
        match (it_1.peek(), it_2.peek()) {
            (Some(&v1), Some(&v2)) => {
                if *v1 < *v2 {
                    *it_1.next().unwrap()
                } else {
                    *it_2.next().unwrap()
                }
            }
            (Some(v1), None) => *it_1.next().unwrap(),
            (None, Some(v2)) => *it_2.next().unwrap(),
            (None,  None) => unreachable!(),
        }
    }

    let mut last_val = 0;

    while curr_num < median {
        last_val = get_less(&mut it_1, &mut it_2);
        curr_num += 1;
    }

    if is_odd {
        get_less(&mut it_1, &mut it_2) as f64
    } else {
        (last_val + get_less(&mut it_1, &mut it_2)) as f64 / 2.0
    }
}
