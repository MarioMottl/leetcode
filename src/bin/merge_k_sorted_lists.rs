use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(PartialEq, Eq)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

/*
 * BinaryHeap Traits needed
 *  Ord -> must be implemented as not all fields including `next` implement them
 *  PartialOrd -> must be implemented as not all fields including `next` implement them
 *  Eq -> can be derived
 *  PartialEq -> can be derived
 * */
impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.val.cmp(&self.val)) // reverse to make min-heap
    }
}
impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.val.cmp(&self.val)
    }
}

pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let mut heap = BinaryHeap::new();

    for list in lists {
        if let Some(node) = list {
            heap.push(node);
        }
    }

    let mut dummy = Box::new(ListNode::new(0));
    let mut tail = &mut dummy;

    while let Some(mut node) = heap.pop() {
        if let Some(next) = node.next.take() {
            heap.push(next);
        }
        tail.next = Some(Box::new(ListNode::new(node.val)));
        tail = tail.next.as_mut().unwrap();
    }

    dummy.next
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper to convert Vec<i32> to linked list
    fn to_linked_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;
        for v in vec {
            tail.next = Some(Box::new(ListNode::new(v)));
            tail = tail.next.as_mut().unwrap();
        }
        dummy.next
    }

    // Helper to convert linked list to Vec<i32> for assertions
    fn to_vec(mut node: Option<Box<ListNode>>) -> Vec<i32> {
        let mut result = Vec::new();
        while let Some(n) = node {
            result.push(n.val);
            node = n.next;
        }
        result
    }

    #[test]
    fn test_merge_k_lists_basic() {
        let input = vec![
            to_linked_list(vec![1, 4, 5]),
            to_linked_list(vec![1, 3, 4]),
            to_linked_list(vec![2, 6]),
        ];

        let merged = merge_k_lists(input);
        assert_eq!(to_vec(merged), vec![1, 1, 2, 3, 4, 4, 5, 6]);
    }

    #[test]
    fn test_merge_k_lists_empty() {
        let input = vec![];
        let merged = merge_k_lists(input);
        assert_eq!(to_vec(merged), vec![]);
    }

    #[test]
    fn test_merge_k_lists_only_empty_lists() {
        let input = vec![None, None];
        let merged = merge_k_lists(input);
        assert_eq!(to_vec(merged), vec![]);
    }

    #[test]
    fn test_merge_k_lists_mixed_empty() {
        let input = vec![
            None,
            to_linked_list(vec![2]),
            None,
            to_linked_list(vec![-1, 3]),
        ];
        let merged = merge_k_lists(input);
        assert_eq!(to_vec(merged), vec![-1, 2, 3]);
    }

    #[test]
    fn test_merge_k_lists_single_list() {
        let input = vec![to_linked_list(vec![1, 2, 3])];
        let merged = merge_k_lists(input);
        assert_eq!(to_vec(merged), vec![1, 2, 3]);
    }
}
