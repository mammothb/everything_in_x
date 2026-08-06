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
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut count = 0i32;
        let mut curr = head.as_ref();
        while let Some(node) = curr {
            curr = node.next.as_ref();
            count += 1;
        }
        if count == n {
            return head.unwrap().next;
        }

        let mut curr = head.as_mut();
        while count > n + 1 {
            curr = curr.unwrap().next.as_mut();
            count -= 1;
        }
        if let Some(node) = curr {
            let target = node.next.take();
            node.next = target.and_then(|t| t.next);
        }
        head
    }
}

fn main() {}
