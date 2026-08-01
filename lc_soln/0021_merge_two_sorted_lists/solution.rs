struct Solution;

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//     pub val: i32,
//     pub next: Option<Box<ListNode>>,
// }
//
// impl ListNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         ListNode { next: None, val }
//     }
// }

impl Solution {
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = Box::new(ListNode::new(0));
        let mut curr = &mut head;
        while list1.is_some() && list2.is_some() {
            let pick = if list1.as_ref().unwrap().val < list2.as_ref().unwrap().val {
                &mut list1
            } else {
                &mut list2
            };
            let mut node = pick.take().unwrap();
            *pick = node.next.take();
            curr.next = Some(node);
            curr = curr.next.as_mut().unwrap();
        }
        curr.next = if list1.is_some() { list1 } else { list2 };
        head.next
    }
}

fn main() {}
