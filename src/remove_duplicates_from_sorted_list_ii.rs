// https://leetcode.com/problems/remove-duplicates-from-sorted-list-ii/description/
// Given the head of a sorted linked list, delete all nodes that have duplicate numbers,
// leaving only distinct numbers from the original list. Return the linked list sorted as well.

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

pub fn delete_duplicates(head: Option<Rc<RefCell<ListNode>>>) -> Option<Rc<RefCell<ListNode>>> {
    let dummy = Rc::new(RefCell::new(ListNode { val: 0, next: head }));
    let mut previous = dummy.clone();
    loop {
        let current = previous.borrow().next.clone();
        let Some(c) = current.clone() else {
            break;
        };

        let mut has_duplicate = false;
        let mut next = c.borrow().next.clone();
        while let Some(n) = next.clone() {
            if c.borrow().val == n.borrow().val {
                next = n.borrow().next.clone();
                has_duplicate = true;
            } else {
                break;
            }
        }
        if has_duplicate {
            previous.borrow_mut().next = next.clone();
        } else {
            previous = c;
        }
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
        input, expected,
        case(&[], &[]),
        case(&[1], &[1]),
        case(&[1, 1], &[]),
        case(&[1, 1, 2], &[2]),
        case(&[1, 2, 3, 3, 4, 4, 5], &[1, 2, 5]),
        case(&[1, 1, 1, 2, 3], &[2, 3]),
        case(&[1, 2, 2, 3, 4, 4, 5, 5, 6], &[1, 3, 6]),
        case(&[0, 0, 0, 0], &[]),
        case(&[-3, -3, -2, -1, -1, 0, 1, 1, 2], &[-2, 0, 2]),
        case(&[-100, -100, -50, -50, 0, 100, 100], &[0])
    )]
    fn test_delete_duplicates(input: &[i32], expected: &[i32]) {
        let head = build_list(input);
        let result = delete_duplicates(head);
        assert_eq!(list_to_vec(result), expected);
    }
}
