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
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut result = i32::MIN;
        Self::dfs(&root, &mut result);
        result
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, result: &mut i32) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                let node = node.borrow();

                let left_tree = Self::dfs(&node.left, result);
                let right_tree = Self::dfs(&node.right, result);

                let max_single = node.val.max(node.val + left_tree.max(right_tree).max(0));
                let max_sum = max_single.max(node.val + left_tree + right_tree);
                *result = (*result).max(max_sum);
                max_single
            }
        }
    }
}

fn main() {}
