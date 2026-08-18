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
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let root = root?;
        let mut stack = vec![root.clone()];
        while let Some(node) = stack.pop() {
            let n = &mut *node.borrow_mut();
            std::mem::swap(&mut n.left, &mut n.right);
            if let Some(l) = n.left.as_ref() {
                stack.push(l.clone());
            }
            if let Some(r) = n.right.as_ref() {
                stack.push(r.clone());
            }
        }
        Some(root)
    }
}

fn main() {}
