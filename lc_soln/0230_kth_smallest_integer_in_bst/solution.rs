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
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut result = [k, 0];
        Self::dfs(&root, &mut result);
        result[1]
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, result: &mut [i32; 2]) {
        if let Some(node) = root {
            let node = node.borrow();

            Self::dfs(&node.left, result);
            if result[0] == 0 {
                return;
            }

            result[0] -= 1;
            if result[0] == 0 {
                result[1] = node.val;
                return;
            }

            Self::dfs(&node.right, result);
        }
    }
}

fn main() {}
