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

/*
 * NOTE: First solution was using zip --> zip only consumes the first pair so I need a while is_some()
 * loop
 * */

#[allow(unused)]
pub fn add_two_numbers(
    mut l1: Option<Box<ListNode>>,
    mut l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut carry = 0;
    let mut head: Option<Box<ListNode>> = None;
    let mut tail = &mut head;

    while l1.is_some() || l2.is_some() || carry > 0 {
        let sum = match (l1.take(), l2.take()) {
            (Some(node1), Some(node2)) => {
                l1 = node1.next;
                l2 = node2.next;
                node1.val + node2.val + carry
            }
            (Some(node1), None) => {
                l1 = node1.next;
                node1.val + carry
            }
            (None, Some(node2)) => {
                l2 = node2.next;
                node2.val + carry
            }
            (None, None) => carry,
        };

        carry = sum / 10;
        *tail = Some(Box::new(ListNode::new(sum % 10)));
        tail = &mut tail.as_mut().unwrap().next;
    }

    head
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_numbers_example_1() {
        let l1 = vec_to_list(vec![2, 4, 3]);
        let l2 = vec_to_list(vec![5, 6, 4]);
        let result = add_two_numbers(l1, l2);
        println!("{:#?}", result);
        assert_eq!(list_to_vec(result), vec![7, 0, 8]);
    }

    #[test]
    fn add_two_numbers_example_2() {
        let l1 = vec_to_list(vec![0]);
        let l2 = vec_to_list(vec![0]);
        let result = add_two_numbers(l1, l2);
        assert_eq!(list_to_vec(result), vec![0]);
    }

    #[test]
    fn add_two_numbers_example_3() {
        let l1 = vec_to_list(vec![9, 9, 9, 9, 9, 9, 9]);
        let l2 = vec_to_list(vec![9, 9, 9, 9]);
        let result = add_two_numbers(l1, l2);
        assert_eq!(list_to_vec(result), vec![8, 9, 9, 9, 0, 0, 0, 1]);
    }
}
