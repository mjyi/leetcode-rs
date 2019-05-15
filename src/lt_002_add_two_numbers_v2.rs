/*
 * You are given two non-empty linked lists representing two non-negative integers.
 * The digits are stored in reverse order and each of their nodes contain a single digit.
 * Add the two numbers and return it as a linked list.
 *
 * You may assume the two numbers do not contain any leading zero, except the number 0 itself.
 *
 * Example:
 *   Input: (2 -> 4 -> 3) + (5 -> 6 -> 4)
 *   Output: 7 -> 0 -> 8
 *   Explanation: 342 + 465 = 807.
 *
 *  Runtime: 4ms, Memory: 2.3M
 */

use crate::utils::list_node::*;

pub struct Solution;

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::add_lists(&l1, &l2, 0)
    }

    fn add_lists(l1: &Option<Box<ListNode>>, l2: &Option<Box<ListNode>>, carry: i32) -> Option<Box<ListNode>> {
        match (l1, l2, carry) {
            (None, None, 0) => None,
            (None, None, c) => Some(Box::new(ListNode::new(c))),
            (Some(l), None, _) | (None, Some(l), _) => {
                let sum = l.val+carry;
                Some(Box::new(ListNode{val: sum%10, next: Self::add_lists(&l.next,&None,sum/10)}))},
            (Some(l1), Some(l2), _) => {
                let sum = l1.val+l2.val+carry;
                Some(Box::new(ListNode{val: sum%10, next: Self::add_lists(&l1.next,&l2.next,sum/10)}))}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::add_two_numbers(to_list(vec![2, 4, 3]), to_list(vec![5, 6, 4])),
            to_list(vec![7, 0, 8])
        );

        assert_eq!(
            Solution::add_two_numbers(to_list(vec![9, 9, 9, 9]), to_list(vec![9, 9, 9, 9, 9, 9])),
            to_list(vec![8, 9, 9, 9, 0, 0, 1])
        );

        assert_eq!(
            Solution::add_two_numbers(to_list(vec![0]), to_list(vec![0])),
            to_list(vec![0])
        )
    }
}
