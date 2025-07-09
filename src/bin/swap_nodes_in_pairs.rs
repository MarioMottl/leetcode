#[derive(PartialEq, Eq, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

#[allow(unused)]
impl ListNode {
    fn new(val: i32) -> Self {
        Self { val, next: None }
    }
}

pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
    let mut prev = dummy.as_mut();

    while let Some(mut first) = prev.as_mut()?.next.take() {
        if let Some(mut second) = first.next.take() {
            // Save what's after the second node
            let next_pair = second.next.take();

            // Swap first and second
            second.next = Some(first);
            prev.as_mut()?.next = Some(second);

            // Move `prev` forward to the new pair's tail (which is `first`)
            prev = prev?.next.as_mut()?.next.as_mut();
            // Reattach the remaining list
            prev.as_mut()?.next = next_pair;
        } else {
            // Only one node left, reattach it and we're done
            prev.as_mut()?.next = Some(first);
            break;
        }
    }

    dummy.unwrap().next
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
    fn test_even_length_list() {
        let input = to_linked_list(vec![1, 2, 3, 4]);
        let result = swap_pairs(input);
        assert_eq!(to_vec(result), vec![2, 1, 4, 3]);
    }

    #[test]
    fn test_odd_length_list() {
        let input = to_linked_list(vec![1, 2, 3]);
        let result = swap_pairs(input);
        assert_eq!(to_vec(result), vec![2, 1, 3]);
    }

    #[test]
    fn test_single_element() {
        let input = to_linked_list(vec![1]);
        let result = swap_pairs(input);
        assert_eq!(to_vec(result), vec![1]);
    }

    #[test]
    fn test_empty_list() {
        let input = to_linked_list(vec![]);
        let result = swap_pairs(input);
        assert_eq!(to_vec(result), vec![]);
    }

    #[test]
    fn test_longer_list() {
        let input = to_linked_list(vec![1, 2, 3, 4, 5, 6, 7]);
        let result = swap_pairs(input);
        assert_eq!(to_vec(result), vec![2, 1, 4, 3, 6, 5, 7]);
    }
}
