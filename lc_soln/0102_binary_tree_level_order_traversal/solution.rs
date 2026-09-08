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
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut q = VecDeque::new();
        if let Some(node) = root {
            q.push_back(node);
        }

        while !q.is_empty() {
            let n = q.len();
            let mut row = Vec::new();
            for _ in 0..n {
                let node = q.pop_front().unwrap();
                let node = node.borrow();
                row.push(node.val);
                if let Some(ref left) = node.left {
                    q.push_back(Rc::clone(left));
                }
                if let Some(ref right) = node.right {
                    q.push_back(Rc::clone(right));
                }
            }
            result.push(row);
        }
        result
    }
}

fn main() {}
