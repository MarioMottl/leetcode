#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

#[allow(unused)]
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    use std::collections::HashSet;

    let mut remove_set = HashSet::new();
    let mut dummy = Box::new(ListNode { val: 0, next: head });
    let mut current = &mut dummy;

    while let Some(mut node) = current.next.take() {
        if remove_set.contains(&node.val) {
            // Skip the duplicate
            current.next = node.next.take();
        } else {
            // Keep the value and move forward
            remove_set.insert(node.val);
            current.next = Some(node);
            current = current.next.as_mut().unwrap();
        }
    }

    dummy.next
}

fn main() {}

#[cfg(test)]
mod tests {

    use super::*;

    fn to_linked_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut current = &mut dummy;
        for val in vec {
            current.next = Some(Box::new(ListNode::new(val)));
            current = current.next.as_mut().unwrap();
        }
        dummy.next
    }

    fn to_vec(mut node: Option<Box<ListNode>>) -> Vec<i32> {
        let mut result = Vec::new();
        while let Some(n) = node {
            result.push(n.val);
            node = n.next;
        }
        result
    }

    #[test]
    fn test_delete_ones() {
        let input = to_linked_list(vec![1, 1, 2]);
        let result = delete_duplicates(input);
        assert_eq!(to_vec(result), vec![1, 2]);
    }
}
