use std::rc::Rc;
use std::cell::RefCell;
use crate::TreeNode;

struct Solution {}

impl Solution {
    pub fn merge_trees(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn merge(root1: &mut Option<Rc<RefCell<TreeNode>>>, root2: &Option<Rc<RefCell<TreeNode>>>) {
            if let Some(node1) = root1.as_ref(){
                if let Some(node2) = root2.as_ref(){
                    node1.borrow_mut().val += node2.borrow().val;
                    merge(&mut node1.borrow_mut().left,&node2.borrow_mut().left);
                    merge(&mut node1.borrow_mut().right,&node2.borrow_mut().right);
                }
            }else {
                *root1 = root2.clone();
            }
        }
        let mut root1 = root1;
        merge(&mut root1, &root2);
        root1
    }
}