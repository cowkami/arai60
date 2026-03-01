// https://leetcode.com/problems/add-two-numbers/description/
// You are given two non-empty linked lists representing two non-negative integers.
// The digits are stored in reverse order, and each of their nodes contains a single digit.
// Add the two numbers and return the sum as a linked list.

use std::cell::{Ref, RefCell};
use std::iter::{successors, zip};
use std::rc::Rc;

// Definition for singly-linked list.
// Using Rc<RefCell<ListNode>> for shared mutable ownership.
#[derive(Debug, PartialEq, Eq)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Rc<RefCell<ListNode>>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

pub fn add_two_numbers(
    l1: Option<Rc<RefCell<ListNode>>>,
    l2: Option<Rc<RefCell<ListNode>>>,
) -> Option<Rc<RefCell<ListNode>>> {
    let mut num1 = l1.clone();
    let mut num2 = l2.clone();
    let dummy = Rc::new(RefCell::new(ListNode::new(0)));
    let mut current = dummy.clone();
    let mut prev_tenth = 0;

    loop {
        if num1.is_none() && num2.is_none() && prev_tenth == 0 {
            break;
        }

        let n1 = num1.unwrap_or(Rc::new(RefCell::new(ListNode::new(0))));
        let n2 = num2.unwrap_or(Rc::new(RefCell::new(ListNode::new(0))));

        let val = n1.borrow().val + n2.borrow().val + prev_tenth;
        let new_val = val % 10;
        prev_tenth = val / 10;
        current.borrow_mut().next = Some(Rc::new(RefCell::new(ListNode::new(new_val))));
        let c = current.borrow().next.clone().unwrap();
        current = c;

        num1 = n1.borrow().next.clone();
        num2 = n2.borrow().next.clone();
    }
    dummy.borrow().next.clone()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    fn build_list(values: &[i32]) -> Option<Rc<RefCell<ListNode>>> {
        if values.is_empty() {
            return None;
        }

        let mut nodes: Vec<Rc<RefCell<ListNode>>> = Vec::with_capacity(values.len());
        for &val in values {
            nodes.push(Rc::new(RefCell::new(ListNode::new(val))));
        }
        for i in 0..values.len() - 1 {
            let next_node = nodes[i + 1].clone();
            nodes[i].borrow_mut().next = Some(next_node);
        }
        Some(nodes[0].clone())
    }

    fn list_to_vec(mut head: Option<Rc<RefCell<ListNode>>>) -> Vec<i32> {
        let mut out = Vec::new();
        while let Some(node) = head {
            out.push(node.borrow().val);
            head = node.borrow().next.clone();
        }
        out
    }

    #[rstest(
        l1, l2, expected,
        case(&[2, 4, 3], &[5, 6, 4], &[7, 0, 8]),
        case(&[0], &[0], &[0]),
        case(&[9, 9, 9, 9, 9, 9, 9], &[9, 9, 9, 9], &[8, 9, 9, 9, 0, 0, 0, 1]),
        case(&[1], &[9, 9], &[0, 0, 1]),
        case(&[5], &[5], &[0, 1]),
        case(&[1, 8], &[0], &[1, 8]),
        case(&[0], &[1, 8], &[1, 8]),
        case(&[9], &[1], &[0, 1]),
        case(&[9, 9], &[1], &[0, 0, 1]),
        case(&[8, 9, 9], &[7], &[5, 0, 0, 1]),
        case(&[3, 0, 0, 1], &[7, 9], &[0, 0, 1, 1]),
        case(&[6, 1, 6], &[5, 9, 2], &[1, 1, 9]),
        case(&[9, 9, 9], &[9, 9, 9], &[8, 9, 9, 1]),
        case(&[0], &[9, 9, 9, 9], &[9, 9, 9, 9])
    )]
    fn test_add_two_numbers(l1: &[i32], l2: &[i32], expected: &[i32]) {
        let l1 = build_list(l1);
        let l2 = build_list(l2);
        let result = add_two_numbers(l1, l2);
        assert_eq!(list_to_vec(result), expected);
    }
}
