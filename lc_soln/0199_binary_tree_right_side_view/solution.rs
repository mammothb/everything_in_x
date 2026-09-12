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
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        if let Some(r) = root {
            let mut q = VecDeque::new();
            q.push_back(r);

            while !q.is_empty() {
                let n = q.len();
                for i in 0..n {
                    if let Some(node) = q.pop_front() {
                        let node = node.borrow();
                        if i == n - 1 {
                            result.push(node.val);
                        }
                        if let Some(left) = &node.left {
                            q.push_back(left.clone());
                        }
                        if let Some(right) = &node.right {
                            q.push_back(right.clone());
                        }
                    }
                }
            }
        }
        result
    }
}

fn main() {}
