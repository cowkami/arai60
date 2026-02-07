// https://leetcode.com/problems/linked-list-cycle/description/
// Given head, the head of a linked list, determine if the linked list has a cycle in it.
//
// There is a cycle in a linked list if there is some node in the list that can be reached again by continuously following the next pointer.
// Internally, pos is used to denote the index of the node that tail's next pointer is connected to. Note that pos is not passed as a parameter.
//
// Return true if there is a cycle in the linked list. Otherwise, return false.


use std::iter::{successors, zip};
use std::rc::Rc;
use std::cell::RefCell;

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
        ListNode {
            val,
            next: None,
        }
    }
}

// Implement Floyd's Tortoise and Hare algorithm here
pub fn has_cycle(head: Option<Rc<RefCell<ListNode>>>) -> bool {
    let mut tortoise = head.clone();
    let mut hare = head.clone().and_then(|n| n.borrow().next.clone());

    while let (Some(t), Some(h)) = (tortoise, hare) {
        if std::ptr::eq(t.as_ptr(), h.as_ptr()) {
            return true;
        }

        tortoise = t.borrow().next.clone();
        hare = h.borrow().next.clone().and_then(|n| n.borrow().next.clone());
    }

    false
}

pub fn has_cycle_iter(head: Option<Rc<RefCell<ListNode>>>) -> bool {
    // advance one step
    let slow = successors(
        head.clone(),
        |n| n.borrow().next.clone()
    );
    // advance two steps
    let mut fast = successors(
        head.clone(),
        |n| n.borrow().next.clone().and_then(
            |n| n.borrow().next.clone()
        )
    );

    fast.next();

    zip(slow, fast).any(|(s, f)| std::ptr::eq(s.as_ptr(), f.as_ptr()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest; // Import rstest within the tests module as well

    // Helper function to create a linked list from a vector with an optional cycle.
    // 'pos' is the 0-indexed position where the tail should connect to form a cycle.
    // If pos is -1, no cycle is created.
    fn build_list_with_cycle(arr: &[i32], pos: i32) -> Option<Rc<RefCell<ListNode>>> {
        if arr.is_empty() {
            return None;
        }

        let mut nodes: Vec<Option<Rc<RefCell<ListNode>>>> = Vec::with_capacity(arr.len());

        // Create all nodes first
        for &val in arr {
            nodes.push(Some(Rc::new(RefCell::new(ListNode::new(val)))));
        }

        // Link nodes
        for i in 0..arr.len() {
            if i + 1 < arr.len() {
                let next_node = nodes[i + 1].clone();
                nodes[i].as_ref().unwrap().borrow_mut().next = next_node;
            }
        }

        // Create cycle if pos is valid
        if pos != -1 && (pos as usize) < arr.len() {
            let cycle_target = nodes[pos as usize].clone();
            // Find the last node and connect its next to the cycle_target
            if let Some(tail_node_option) = nodes.last() {
                if let Some(tail_node_rc) = tail_node_option {
                    tail_node_rc.borrow_mut().next = cycle_target;
                }
            }
        }

        nodes[0].clone() // Return the head of the list
    }

    #[rstest(
        input_list, pos, expected,
        // No Cycle Scenarios
        case(&[], -1, false), // Empty list
        case(&[1], -1, false), // Single node, no cycle
        case(&[1, 2, 3, 4, 5], -1, false), // Multiple nodes, no cycle
        case(&[1; 1000], -1, false), // Long list, no cycle

        // Cycle Scenarios (Existing)
        case(&[1], 0, true), // Single node, loop to itself
        case(&[1, 2], 0, true), // Two nodes, 2 -> 1
        case(&[3, 2, 0, -4], 1, true), // Cycle in middle: -4 -> 2
        case(&[1, 2, 3], 0, true), // Cycle end to head: 3 -> 1
        case(&[0; 1000], 500, true), // Long list, cycle in middle

        // Adversarial Cycle Scenarios (New)
        case(&[1, 2, 3, 4, 5], 4, true), // Short list, cycle at last node (5 -> 5)
        case(&[1, 2, 3, 4, 5], 3, true), // Short list, cycle 5 -> 4 -> 3
        case(&{ let mut v = vec![0; 1000]; for i in 0..1000 { v[i] = i as i32; } v }, 999, true), // Long list, cycle at last node (999 -> 999)
        case(&{ let mut v = vec![0; 1000]; for i in 0..1000 { v[i] = i as i32; } v }, 900, true), // Long list, cycle starts very late
        case(&{ let mut v = vec![0; 2]; for i in 0..2 { v[i] = i as i32; } v }, 1, true) // List [0, 1], 1 -> 1
    )]
    fn test_has_cycle(input_list: &[i32], pos: i32, expected: bool) {
        let head = build_list_with_cycle(input_list, pos);
        assert_eq!(has_cycle(head), expected);
    }

    #[rstest(
        input_list, pos, expected,
        // No Cycle Scenarios
        case(&[], -1, false), // Empty list
        case(&[1], -1, false), // Single node, no cycle
        case(&[1, 2, 3, 4, 5], -1, false), // Multiple nodes, no cycle
        case(&[1; 1000], -1, false), // Long list, no cycle

        // Cycle Scenarios (Existing)
        case(&[1], 0, true), // Single node, loop to itself
        case(&[1, 2], 0, true), // Two nodes, 2 -> 1
        case(&[3, 2, 0, -4], 1, true), // Cycle in middle: -4 -> 2
        case(&[1, 2, 3], 0, true), // Cycle end to head: 3 -> 1
        case(&[0; 1000], 500, true), // Long list, cycle in middle

        // Adversarial Cycle Scenarios (New)
        case(&[1, 2, 3, 4, 5], 4, true), // Short list, cycle at last node (5 -> 5)
        case(&[1, 2, 3, 4, 5], 3, true), // Short list, cycle 5 -> 4 -> 3
        case(&{ let mut v = vec![0; 1000]; for i in 0..1000 { v[i] = i as i32; } v }, 999, true), // Long list, cycle at last node (999 -> 999)
        case(&{ let mut v = vec![0; 1000]; for i in 0..1000 { v[i] = i as i32; } v }, 900, true), // Long list, cycle starts very late
        case(&{ let mut v = vec![0; 2]; for i in 0..2 { v[i] = i as i32; } v }, 1, true) // List [0, 1], 1 -> 1
    )]
    fn test_has_cycle_iter(input_list: &[i32], pos: i32, expected: bool) {
        let head = build_list_with_cycle(input_list, pos);
        assert_eq!(has_cycle_iter(head), expected);
    }
}
