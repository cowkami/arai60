// https://leetcode.com/problems/remove-duplicates-from-sorted-list/description/
// Given the head of a sorted linked list, delete all duplicates such that each element appears only once.
// Return the linked list sorted as well.

use std::cell::RefCell;
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

pub fn delete_duplicates_first(
    head: Option<Rc<RefCell<ListNode>>>,
) -> Option<Rc<RefCell<ListNode>>> {
    let mut current = head.clone();
    while let (Some(c), Some(n)) = (
        current.clone(),
        current.and_then(|n| n.borrow().next.clone()),
    ) {
        if c.borrow().val == n.borrow().val {
            c.borrow_mut().next = n.borrow().next.clone();
            current = Some(c);
        } else {
            current = c.borrow().next.clone();
        }
    }
    head
}

pub fn delete_duplicates(head: Option<Rc<RefCell<ListNode>>>) -> Option<Rc<RefCell<ListNode>>> {
    let mut current = head.clone();
    while let (Some(c), Some(n)) = (
        current.clone(),
        current.clone().and_then(|n| n.borrow().next.clone()),
    ) {
        if c.borrow().val == n.borrow().val {
            c.borrow_mut().next = n.borrow().next.clone();
        } else {
            current = c.borrow().next.clone();
        }
    }
    head
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
        input, expected,
        case(&[], &[]),
        case(&[1], &[1]),
        case(&[1, 1], &[1]),
        case(&[1, 1, 2], &[1, 2]),
        case(&[1, 1, 2, 3, 3], &[1, 2, 3]),
        case(&[1, 2, 3], &[1, 2, 3]),
        case(&[0, 0, 0, 0], &[0]),
        case(&[-1, -1, 0, 0, 0, 2, 2], &[-1, 0, 2]),
        case(&[-100, -100, -50, -50, 0, 100, 100], &[-100, -50, 0, 100]),
        case(&[1, 1, 1, 2, 3, 3, 4, 4, 4], &[1, 2, 3, 4])
    )]
    fn test_delete_duplicates(input: &[i32], expected: &[i32]) {
        let head = build_list(input);
        let result = delete_duplicates(head);
        assert_eq!(list_to_vec(result), expected);
    }
}
