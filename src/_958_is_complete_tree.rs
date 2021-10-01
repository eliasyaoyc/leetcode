use std::cell::RefCell;
use std::rc::Rc;
use crate::TreeNode;

struct Solution {}

impl Solution {
    pub fn is_complete_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn depth(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
            if let Some(node) = root.as_ref() {
                let left = depth(&node.borrow().left);
                let right = depth(&node.borrow().right);
                1 + i32::max(left,right)
            } else { 0 }
        }
        let mut index = 1;
        let len = depth(&root);

        let mut queued = vec![root];
        while !queued.is_empty() && index <= len{
            if index == len {
                if let Some(pos) = queued.iter().position(|node| node.is_none()) {
                    if let Some(_) = &queued[pos..].to_vec().iter().find(|node| node.is_some()) {
                        return false;
                    }else {
                        return true;
                    }
                } else {
                    return true;
                }
            } else {
                if let Some(_) = queued.iter().find(|node| node.is_none()) {
                    return false;
                } else {
                    let ret = queued.iter().fold(vec![], |mut acc, node| {
                        let node = node.clone().unwrap();
                        let left = node.borrow().left.clone();
                        let right = node.borrow().right.clone();
                        acc.push(left);
                        acc.push(right);
                        acc
                    });
                    queued = ret;
                }
            }
            index += 1;
        }
        true
    }
}

#[test]
fn t() {
    let mut root = TreeNode::new(1);
    // let mut root_left = TreeNode::new(2);
    let mut root_right = TreeNode::new(2);

    // let mut root_right_right = TreeNode::new(6);


    // root_left.left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    // root_left.right = Some(Rc::new(RefCell::new(TreeNode::new(5))));

    // root_right.left = Some(Rc::new(RefCell::new(root_right_right)));

    // root.left = Some(Rc::new(RefCell::new(root_left)));
    root.right = Some(Rc::new(RefCell::new(root_right)));

    let result = Solution::is_complete_tree(Some(Rc::new(RefCell::new(root))));
    println!("{}",result);
}