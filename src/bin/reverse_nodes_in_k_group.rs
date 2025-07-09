#[derive(PartialEq, Eq)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

#[allow(unused)]
impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut count = 0;
    let mut ptr = &head;

    // Step 1: Check if we have at least k nodes
    while let Some(node) = ptr {
        count += 1;
        if count == k {
            break;
        }
        ptr = &node.next;
    }

    if count < k {
        return head; // Not enough nodes to reverse
    }

    // Step 2: Reverse k nodes
    let mut prev = None;
    let mut curr = head.take();
    for _ in 0..k {
        if let Some(mut node) = curr {
            let next = node.next.take();
            node.next = prev;
            prev = Some(node);
            curr = next;
        }
    }

    // Step 3: Connect the last node of reversed part to next group
    if let Some(ref mut node) = prev {
        let mut tail = node;
        while let Some(ref mut next) = tail.next {
            tail = next;
        }
        tail.next = reverse_k_group(curr, k);
    }

    prev
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    // Helper: convert Vec<i32> -> Linked List
    fn from_vec(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        for &val in vec.iter().rev() {
            let mut node = Box::new(ListNode::new(val));
            node.next = head;
            head = Some(node);
        }
        head
    }

    // Helper: convert Linked List -> Vec<i32>
    fn to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut result = Vec::new();
        while let Some(node) = head {
            result.push(node.val);
            head = node.next;
        }
        result
    }

    #[test]
    fn test_k_equals_2() {
        let input = from_vec(vec![1, 2, 3, 4, 5]);
        let output = reverse_k_group(input, 2);
        assert_eq!(to_vec(output), vec![2, 1, 4, 3, 5]);
    }

    #[test]
    fn test_k_equals_3() {
        let input = from_vec(vec![1, 2, 3, 4, 5]);
        let output = reverse_k_group(input, 3);
        assert_eq!(to_vec(output), vec![3, 2, 1, 4, 5]);
    }

    #[test]
    fn test_k_equals_1() {
        let input = from_vec(vec![1, 2, 3]);
        let output = reverse_k_group(input, 1);
        assert_eq!(to_vec(output), vec![1, 2, 3]);
    }

    #[test]
    fn test_k_equals_list_length() {
        let input = from_vec(vec![1, 2, 3]);
        let output = reverse_k_group(input, 3);
        assert_eq!(to_vec(output), vec![3, 2, 1]);
    }

    #[test]
    fn test_k_greater_than_list_length() {
        let input = from_vec(vec![1, 2]);
        let output = reverse_k_group(input, 3);
        assert_eq!(to_vec(output), vec![1, 2]);
    }

    #[test]
    fn test_empty_list() {
        let input = from_vec(vec![]);
        let output = reverse_k_group(input, 2);
        assert_eq!(to_vec(output), vec![]);
    }

    #[test]
    fn test_single_element_list() {
        let input = from_vec(vec![42]);
        let output = reverse_k_group(input, 2);
        assert_eq!(to_vec(output), vec![42]);
    }
}
