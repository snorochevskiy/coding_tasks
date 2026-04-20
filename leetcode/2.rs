// 2. Add Two Numbers
//
// You are given two non-empty linked lists representing two non-negative integers.
// The digits are stored in reverse order, and each of their nodes contains a single digit.
// Add the two numbers and return the sum as a linked list.
// You may assume the two numbers do not contain any leading zero, except the number 0 itself.
//
// Example 1:
// (2) -> (4) -> (3)
// (5) -> (6) -> (4)
// -----------------
// (7) -> (0) -> (8)
//
// Input: l1 = [2,4,3], l2 = [5,6,4]
// Output: [7,0,8]
// Explanation: 342 + 465 = 807.
//
// Example 2:
//   Input: l1 = [0], l2 = [0]
//   Output: [0]
//
// Example 3:
//   Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
//   Output: [8,9,9,9,0,0,0,1]
//
// Constraints:
//   The number of nodes in each linked list is in the range [1, 100].
//   0 <= Node.val <= 9
//   It is guaranteed that the list represents a number that does not have leading zeros.


fn main() {
    assert_eq!(
        add_two_numbers(
            Some(Box::new(ListNode { val: 2, next: Some(Box::new(ListNode { val: 4, next: Some(Box::new(ListNode { val: 3, next: None })) })) })),
            Some(Box::new(ListNode { val: 5, next: Some(Box::new(ListNode { val: 6, next: Some(Box::new(ListNode { val: 4, next: None })) })) })),
        ),
        Some(Box::new(ListNode { val: 7, next: Some(Box::new(ListNode { val: 0, next: Some(Box::new(ListNode { val: 8, next: None })) })) })),
    )
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut carry = 0;
    let mut a1 = l1;
    let mut a2 = l2;
    let mut res: Vec<i32> = Vec::new();
    while a1.is_some() || a2.is_some() {
        let a1_val = a1.as_ref().map(|a|a.val).unwrap_or(0);
        let a2_val= a2.as_ref().map(|a|a.val).unwrap_or(0);
        let mut next = a1_val + a2_val + carry;
        a1 = a1.and_then(|a|a.next);
        a2 = a2.and_then(|a|a.next);
        carry = 0;
        if next > 9 {
            carry = 1;
            next = next - 10;
        }
        res.push(next);
    }
    if carry > 0 {
        res.push(carry);
    }

    res.into_iter()
        .rev()
        .fold(None as Option<Box<ListNode>>, |acc, elem|
            Some(Box::new(ListNode { val: elem,  next: acc }))
        )
}

