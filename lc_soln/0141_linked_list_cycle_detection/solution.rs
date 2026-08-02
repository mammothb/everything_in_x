struct Solution;

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//     pub val: i32,
//     pub next: *mut ListNode,
// }
//
// impl ListNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         ListNode { next: std::ptr::null_mut(), val }
//     }
// }

impl Solution {
    pub fn has_cycle(head: *mut ListNode) -> bool {
        let mut fast = head;
        let mut slow = head;
        while !fast.is_null() && unsafe { (*fast).next } != std::ptr::null_mut() {
            slow = unsafe { (*slow).next };
            fast = unsafe { (*(*fast).next).next };
            if slow == fast {
                return true;
            }
        }
        false
    }
}

fn main() {}
