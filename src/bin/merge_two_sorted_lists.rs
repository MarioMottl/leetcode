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

#[allow(unused)]
fn merge_two_lists(
    mut list1: Option<Box<ListNode>>,
    mut list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode::new(0));
    let mut tail = &mut dummy;

    while let (Some(l1), Some(l2)) = (&list1, &list2) {
        if l1.val <= l2.val {
            let next = list1.as_mut().unwrap().next.take();
            tail.next = list1;
            list1 = next;
        } else {
            let next = list2.as_mut().unwrap().next.take();
            tail.next = list2;
            list2 = next;
        }
        tail = tail.next.as_mut().unwrap();
    }

    tail.next = if list1.is_some() { list1 } else { list2 };
    dummy.next
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2_lists() {
        let l1 = vec_to_list(vec![1, 2, 4]);
        let l2 = vec_to_list(vec![1, 3, 4]);
        let result = merge_two_lists(l1, l2);
        assert_eq!(list_to_vec(result), vec![1, 1, 2, 3, 4, 4])
    }

    #[test]
    fn test_2_empty_lists() {
        let l1 = vec_to_list(vec![]);
        let l2 = vec_to_list(vec![]);
        let result = merge_two_lists(l1, l2);
        assert_eq!(list_to_vec(result), vec![])
    }

    #[test]
    fn test_2_0_lists() {
        let l1 = vec_to_list(vec![]);
        let l2 = vec_to_list(vec![0]);
        let result = merge_two_lists(l1, l2);
        println!("{:#?}", result);
        assert_eq!(list_to_vec(result), vec![0])
    }
}
