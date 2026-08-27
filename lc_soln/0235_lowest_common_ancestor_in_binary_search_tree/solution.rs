struct Solution;

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//     pub val: i32,
//     pub left: Option<Rc<RefCell<TreeNode>>>,
//     pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         TreeNode {
//             val,
//             left: None,
//             right: None,
//         }
//     }
// }

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let p_val = p.as_ref()?.borrow().val;
        let q_val = q.as_ref()?.borrow().val;
        let mut curr = root;

        while let Some(node) = curr {
            let node_val = node.borrow().val;
            if p_val > node_val && q_val > node_val {
                curr = node.borrow().right.clone();
            } else if p_val < node_val && q_val < node_val {
                curr = node.borrow().left.clone();
            } else {
                return Some(node);
            }
        }
        None
    }
}

fn main() {}
