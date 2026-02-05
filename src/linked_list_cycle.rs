// https://leetcode.com/problems/linked-list-cycle/description/
// Given head, the head of a linked list, determine if the linked list has a cycle in it.
//
// There is a cycle in a linked list if there is some node in the list that can be reached again by continuously following the next pointer.
// Internally, pos is used to denote the index of the node that tail's next pointer is connected to. Note that pos is not passed as a parameter.
//
// Return true if there is a cycle in the linked list. Otherwise, return false.


use std::rc::Rc;
use std::cell::RefCell;
use rstest::rstest; // Import rstest

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

// Placeholder for the function to detect a cycle.
// This will always return false for now, as the focus is on writing tests.
pub fn has_cycle(head: Option<Rc<RefCell<ListNode>>>) -> bool {
    // Implement Floyd's Tortoise and Hare algorithm here
    false // TODO: Replace with actual implementation
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
        // Cycle Scenarios
        case(&[1], 0, true), // Single node, loop to itself
        case(&[1, 2], 0, true), // Two nodes, 2 -> 1
        case(&[3, 2, 0, -4], 1, true), // Cycle in middle: -4 -> 2
        case(&[1, 2, 3], 0, true), // Cycle end to head: 3 -> 1
        case(&[0; 1000], 500, true) // Long list, cycle in middle
    )]
    fn test_has_cycle(input_list: &[i32], pos: i32, expected: bool) {
        let head = build_list_with_cycle(input_list, pos);
        assert_eq!(has_cycle(head), expected);
    }
}