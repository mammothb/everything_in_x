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
    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if root.is_none() && sub_root.is_none() {
            return true;
        }
        if root.is_none() || sub_root.is_none() {
            return false;
        }
        if Self::same_tree(&root, &sub_root) {
            return true;
        }
        let node = root.as_ref().unwrap().borrow();
        Self::is_subtree(node.left.clone(), sub_root.clone())
            || Self::is_subtree(node.right.clone(), sub_root.clone())
    }

    fn same_tree(
        root1: &Option<Rc<RefCell<TreeNode>>>,
        root2: &Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (root1, root2) {
            (None, None) => true,
            (Some(a), Some(b)) => {
                let a = a.borrow();
                let b = b.borrow();
                a.val == b.val
                    && Self::same_tree(&a.left, &b.left)
                    && Self::same_tree(&a.right, &b.right)
            }
            _ => false,
        }
    }
}

fn main() {}
