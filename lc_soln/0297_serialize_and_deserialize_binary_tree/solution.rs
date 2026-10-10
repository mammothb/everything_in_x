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

struct Codec {}

impl Codec {
    fn new() -> Self {
        Self {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut result = vec!["".to_string()];
        Self::dfs_serialize(&root, &mut result);
        result.join(",")
    }

    fn dfs_serialize(root: &Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<String>) {
        match root {
            None => result.push("N".to_string()),
            Some(node) => {
                let node = node.borrow();
                result.push(node.val.to_string());
                Self::dfs_serialize(&node.left, result);
                Self::dfs_serialize(&node.right, result);
            }
        }
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let values: Vec<&str> = data.split(',').collect();
        let mut i = 0;
        Self::dfs_deserialize(&values, &mut i)
    }

    fn dfs_deserialize(values: &[&str], i: &mut usize) -> Option<Rc<RefCell<TreeNode>>> {
        *i += 1;
        if values[*i] == "N" {
            return None;
        }
        let mut root = TreeNode::new(values[*i].parse::<i32>().unwrap());
        root.left = Self::dfs_deserialize(values, i);
        root.right = Self::dfs_deserialize(values, i);
        Some(Rc::new(RefCell::new(root)))
    }
}

fn main() {}
