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

pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev: Option<Box<ListNode>> = None;

    while let Some(mut current) = head {
        head = current.next.take(); // detach current from the rest
        current.next = prev; // reverse the pointer
        prev = Some(current); // move prev forward
    }

    prev
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
}
