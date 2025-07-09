use std::collections::HashSet;

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

pub fn modified_list(nums: Vec<i32>, head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let remove_set: HashSet<i32> = nums.into_iter().collect();
    let mut dummy = Box::new(ListNode { val: 0, next: head });
    let mut current = &mut dummy;

    while let Some(ref mut node) = current.next {
        if remove_set.contains(&node.val) {
            current.next = node.next.take();
        } else {
            current = current.next.as_mut().unwrap();
        }
    }

    dummy.next
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_nodes() {
        fn from_vec(vec: Vec<i32>) -> Option<Box<ListNode>> {
            let mut head = None;
            for &val in vec.iter().rev() {
                let mut node = Box::new(ListNode::new(val));
                node.next = head;
                head = Some(node);
            }
            head
        }

        fn to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
            let mut result = Vec::new();
            while let Some(node) = head {
                result.push(node.val);
                head = node.next;
            }
            result
        }

        let head = from_vec(vec![1, 2, 3, 4, 5]);
        let result = modified_list(vec![1, 2, 3], head);
        assert_eq!(to_vec(result), vec![4, 5]);
    }
}
