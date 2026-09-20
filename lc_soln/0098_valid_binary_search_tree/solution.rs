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
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::dfs(root, i32::MIN, i32::MAX)
    }

    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, min_val: i32, max_val: i32) -> bool {
        match root {
            None => true,
            Some(r) => {
                let node = r.borrow();
                if !(min_val < node.val && node.val < max_val) {
                    return false;
                }
                Self::dfs(node.left.clone(), min_val, node.val)
                    && Self::dfs(node.right.clone(), node.val, max_val)
            }
        }
    }
}

fn main() {}
