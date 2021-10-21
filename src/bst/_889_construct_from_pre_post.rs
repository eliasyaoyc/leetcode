use std::rc::Rc;
use std::cell::RefCell;
use crate::TreeNode;

struct Solution {}

impl Solution {
    pub fn construct_from_pre_post(preorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let L = preorder.len();

        if L == 0 {
            return None;
        }

        let mut root = TreeNode::new(preorder[0]);

        if L == 1 {
            return Some(Rc::new(RefCell::new(root)));
        }

        let left = postorder.iter().position(|&x| x == preorder[1]).unwrap() + 1;

        root.left = Self::construct_from_pre_post(preorder[1..=left].to_vec(), postorder[0..left].to_vec());
        root.right = Self::construct_from_pre_post(preorder[left+1..].to_vec(), postorder[left..L-1].to_vec());

        return Some(Rc::new(RefCell::new(root)));
    }
}