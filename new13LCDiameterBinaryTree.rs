// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
use std::cmp;

impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut diameter = 0;
        Self::dfs(&root,&mut diameter);
        diameter
    }
    fn dfs(node_opt: &Option<Rc<RefCell<TreeNode>>>, diameter:&mut i32) -> i32{
        if let Some(node_rc) = node_opt{
            let node = node_rc.borrow();

            let left_height = Self::dfs(&node.left, diameter);
            let right_height = Self::dfs(&node.right, diameter);

            *diameter = cmp::max(*diameter,left_height+right_height);
            1+cmp::max(left_height,right_height)
        } else {
            0
        }

    }
}
