use std::rc::Rc;
use std::cell::RefCell;
use crate::TreeNode;

struct Solution {}

impl Solution {
    pub fn check_sub_tree(t1: Option<Rc<RefCell<TreeNode>>>, t2: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if t1.is_none() {
            return false;
        }
        if t2.is_none() {
            return true;
        }
        fn dfs(t1: Option<Rc<RefCell<TreeNode>>>, t2: Option<Rc<RefCell<TreeNode>>>) -> bool {
            match (t1, t2) {
                (None, Some(node2)) => return false,
                (Some(node1), None) => return false,
                (Some(node1), Some(node2)) => {
                    let node1_left = node1.borrow_mut().left.take();
                    let node1_right = node1.borrow_mut().right.take();
                    let node2_left = node2.borrow_mut().left.take();
                    let node2_right = node2.borrow_mut().right.take();
                    return node1.borrow().val == node2.borrow().val && dfs(node1_left, node2_left)
                        && dfs(node1_right, node2_right);
                }
                (None, None) => return true,
            }
        }
        let left = t1.clone().unwrap().borrow_mut().left.take();
        let right = t1.clone().unwrap().borrow_mut().right.take();
        dfs(t1, t2.clone()) || Self::check_sub_tree(left, t2.clone()) || Self::check_sub_tree(right, t2)
    }

    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn construct(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>>{
            if preorder.is_empty() || inorder.is_empty() {
                return None;
            }
            let mut node = TreeNode::new(inorder[0]);
            let index = preorder.iter().position(|&v| v == inorder[0]).unwrap();
            node.left = construct(preorder[..index].to_vec(),inorder[1..].to_vec());
            node.right = construct(preorder[index+1..].to_vec(),inorder[1..].to_vec());
            Some(Rc::new(RefCell::new(node)))
        }
        construct(preorder,inorder)
    }
}