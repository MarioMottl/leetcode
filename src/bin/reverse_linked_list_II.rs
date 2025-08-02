#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn reverse_between(
    head: Option<Box<ListNode>>,
    left: i32,
    right: i32,
) -> Option<Box<ListNode>> {
    if left == right {
        return head;
    }

    let mut dummy = Box::new(ListNode { val: 0, next: head });
    let mut prev = &mut dummy;

    for _ in 1..left {
        prev = prev.next.as_mut().unwrap();
    }

    let mut curr = prev.next.take();
    let mut rev: Option<Box<ListNode>> = None;

    for _ in left..=right {
        if let Some(mut node) = curr {
            curr = node.next.take();
            node.next = rev;
            rev = Some(node);
        }
    }

    prev.next = rev;
    let mut tail = prev;
    while tail.next.as_ref().unwrap().next.is_some() {
        tail = tail.next.as_mut().unwrap();
    }
    tail.next.as_mut().unwrap().next = curr;

    dummy.next
}

#[allow(unused)]
fn vec_to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut tail = &mut head;

    for val in vec {
        *tail = Some(Box::new(ListNode::new(val)));
        tail = &mut tail.as_mut().unwrap().next;
    }

    head
}

#[allow(unused)]
fn list_to_vec(mut list: Option<Box<ListNode>>) -> Vec<i32> {
    let mut vec = vec![];
    while let Some(node) = list {
        vec.push(node.val);
        list = node.next;
    }
    vec
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            list_to_vec(reverse_between(vec_to_list(vec![1, 2, 3, 4, 5]), 2, 4)),
            [1, 4, 3, 2, 5]
        )
    }
}
