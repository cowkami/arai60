// https://leetcode.com/problems/linked-list-cycle-ii/description/
// Given the head of a linked list, return the node where the cycle begins.
// If there is no cycle, return None.

use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

// Definition for singly-linked list.
// Using Rc<RefCell<ListNode>> for shared mutable ownership to enable cycles.
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

// Floyd's Tortoise and Hare algorithm to detect cycle start in O(1) space.
pub fn detect_cycle_first_solution(
    head: Option<Rc<RefCell<ListNode>>>,
) -> Option<Rc<RefCell<ListNode>>> {
    // first step, check if cycle exists
    let mut slow = head.clone();
    let mut fast = head.clone().and_then(|n| n.borrow().next.clone());

    let mut end = None;
    while let (Some(s), Some(f)) = (slow, fast) {
        if std::ptr::eq(s.as_ptr(), f.as_ptr()) {
            end = Some(s);
            break;
        }

        slow = s.borrow().next.clone();
        fast = f
            .borrow()
            .next
            .clone()
            .and_then(|n| n.borrow().next.clone());
    }
    let e = if let Some(e) = end {
        e
    } else {
        return None;
    };

    // second step
    let mut start = head.clone();
    let mut result = None;

    'outer: while let Some(s) = start {
        let mut check = e.borrow().next.clone();

        while let Some(c) = check {
            if std::ptr::eq(s.as_ptr(), c.as_ptr()) {
                result = Some(s);
                break 'outer;
            }
            if std::ptr::eq(c.as_ptr(), e.as_ptr()) {
                break;
            }

            check = c.borrow().next.clone();
        }

        if std::ptr::eq(s.as_ptr(), e.as_ptr()) {
            break;
        }

        start = s.borrow().next.clone();
    }

    result
}

pub fn detect_cycle(head: Option<Rc<RefCell<ListNode>>>) -> Option<Rc<RefCell<ListNode>>> {
    // use set as checked nodes. and return node that exists already in the sets.
    // because it's start of loop.
    let mut node = head;
    let mut checked = HashSet::new();
    while let Some(n) = node {
        let node_ptr = n.as_ptr() as *const ListNode;
        if checked.contains(&node_ptr) {
            return Some(n);
        }
        checked.insert(node_ptr);
        node = n.borrow().next.clone();
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    // Returns (head, nodes) where nodes[i] is the i-th node in the list.
    fn build_list_with_cycle(
        arr: &[i32],
        pos: i32,
    ) -> (Option<Rc<RefCell<ListNode>>>, Vec<Rc<RefCell<ListNode>>>) {
        if arr.is_empty() {
            return (None, Vec::new());
        }

        let mut nodes: Vec<Rc<RefCell<ListNode>>> = Vec::with_capacity(arr.len());
        for &val in arr {
            nodes.push(Rc::new(RefCell::new(ListNode::new(val))));
        }

        for i in 0..arr.len() {
            if i + 1 < arr.len() {
                let next_node = nodes[i + 1].clone();
                nodes[i].borrow_mut().next = Some(next_node);
            }
        }

        if pos != -1 && (pos as usize) < nodes.len() {
            let cycle_target = nodes[pos as usize].clone();
            let last = nodes.last().unwrap().clone();
            last.borrow_mut().next = Some(cycle_target);
        }

        (Some(nodes[0].clone()), nodes)
    }

    #[rstest(
        input_list, pos, expected_idx,
        case(&[], -1, None),
        case(&[1], -1, None),
        case(&[1], 0, Some(0)),
        case(&[1, 2], 0, Some(0)),
        case(&[1, 2], 1, Some(1)),
        case(&[3, 2, 0, -4], 1, Some(1)),
        case(&[1, 2, 3, 4, 5], -1, None),
        case(&[1, 2, 3, 4, 5], 2, Some(2)),
        case(&[0; 1000], 500, Some(500)),
        // Adversarial / edge cases
        case(&[10, 20, 30, 40], 3, Some(3)), // cycle at last node (tail -> tail)
        case(&[5, 6, 7, 8, 9], 0, Some(0)), // cycle back to head
        case(&[1, 2, 3, 4, 5, 6, 7, 8], 6, Some(6)), // late cycle start
        case(&[42, 42, 42, 42], 2, Some(2)), // duplicate values shouldn't matter
        case(&{ let mut v = vec![0; 1000]; for i in 0..1000 { v[i] = i as i32; } v }, 999, Some(999)), // max pos
        case(&{ let mut v = vec![0; 1000]; for i in 0..1000 { v[i] = (i as i32) * -1; } v }, 123, Some(123)) // negative values
    )]
    fn test_detect_cycle(input_list: &[i32], pos: i32, expected_idx: Option<usize>) {
        let (head, nodes) = build_list_with_cycle(input_list, pos);
        let result = detect_cycle(head);

        match expected_idx {
            None => assert!(result.is_none()),
            Some(idx) => {
                let expected = nodes[idx].clone();
                let actual = result.expect("expected a cycle start node");
                assert!(std::ptr::eq(expected.as_ptr(), actual.as_ptr()));
            }
        }
    }
}
