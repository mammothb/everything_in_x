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
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut in_to_idx: HashMap<i32, i32> = HashMap::with_capacity(inorder.len());
        for (i, &v) in inorder.iter().enumerate() {
            in_to_idx.insert(v, i as i32);
        }
        let mut pre_idx = 0;
        Self::build(
            &preorder,
            0 as i32,
            inorder.len() as i32 - 1,
            &mut pre_idx,
            &in_to_idx,
        )
    }

    fn build(
        preorder: &[i32],
        in_start: i32,
        in_stop: i32,
        pre_idx: &mut usize,
        in_to_idx: &HashMap<i32, i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if in_start > in_stop {
            return None;
        }
        let root_val = preorder[*pre_idx];
        *pre_idx += 1;
        let mut root = TreeNode::new(root_val);
        root.left = Self::build(
            preorder,
            in_start,
            in_to_idx[&root_val] - 1,
            pre_idx,
            in_to_idx,
        );
        root.right = Self::build(
            preorder,
            in_to_idx[&root_val] + 1,
            in_stop,
            pre_idx,
            in_to_idx,
        );
        Some(Rc::new(RefCell::new(root)))
    }
}

fn main() {}
