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
 *  Runtime: 12 ms， Memory: 2.4 MB
 *
 */

use crate::utils::list_node::*;

pub struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dump_node = Some(Box::new(ListNode::new(0)));

        let mut tail_node = &mut dump_node;

        let (mut l1_end, mut l2_end) = (false, false);
        let (mut l1, mut l2) = (l1, l2);
        let mut overwrite = false;

        loop {
            let l1_val = match l1 {
                Some(l1_node) => {
                    l1 = l1_node.next;
                    l1_node.val
                }
                None => {
                    l1_end = true;
                    0
                }
            };
            let l2_val = match l2 {
                Some(l2_node) => {
                    l2 = l2_node.next;
                    l2_node.val
                }
                None => {
                    l2_end = true;
                    0
                }
            };

            if l1_end && l2_end && !overwrite {
                break;
            }

            let sum = l1_val + l2_val + if overwrite { 1 } else { 0 };
            let sum = if sum >= 10 {
                overwrite = true;
                sum - 10
            } else {
                overwrite = false;
                sum
            };

            let cur_node = Some(Box::new(ListNode::new(sum)));

            tail_node.as_mut().unwrap().next = cur_node;
            tail_node = &mut tail_node.as_mut().unwrap().next
        }
        println!("{:?}", dump_node);
        dump_node.unwrap().next
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
