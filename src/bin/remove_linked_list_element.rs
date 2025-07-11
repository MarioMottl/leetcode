#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode { val: 0, next: head });
    let mut current = &mut dummy;

    while let Some(ref mut node) = current.next {
        if node.val == val {
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

    fn vec_to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut tail = &mut head;

        for val in vec {
            *tail = Some(Box::new(ListNode::new(val)));
            tail = &mut tail.as_mut().unwrap().next;
        }

        head
    }

    fn list_to_vec(mut list: Option<Box<ListNode>>) -> Vec<i32> {
        let mut vec = vec![];
        while let Some(node) = list {
            vec.push(node.val);
            list = node.next;
        }
        vec
    }

    #[test]
    fn test_example_1() {
        let result = vec_to_list(vec![1, 2, 6, 3, 4, 5, 6]);
        assert_eq!(list_to_vec(remove_elements(result, 6)), vec![1, 2, 3, 4, 5])
    }
}
