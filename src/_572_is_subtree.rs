use std::rc::Rc;
use std::cell::RefCell;
use crate::TreeNode;

struct Solution {}

impl Solution {
    pub fn is_subtree(root: Option<Rc<RefCell<TreeNode>>>, sub_root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn is_sametree(s: Option<Rc<RefCell<TreeNode>>>,t: Option<Rc<RefCell<TreeNode>>>) ->bool{
            if s.is_none() && t.is_none() { return true;}
            if s.is_none() || t.is_none() { return false;}
            let s = s.unwrap();
            let t = t.unwrap();

            s == t
        }

        if let Some(s) = root{
            let s_ref = s.borrow();
            if s_ref.val == sub_root.as_ref().unwrap().borrow().val{
                if is_sametree(Some(s.clone()),sub_root.clone()) {
                    return true;
                }
            }
            Self::is_subtree(s_ref.left.clone(),sub_root.clone()) || Self::is_subtree(s_ref.right.clone(),sub_root.clone())
        }else {false}
    }
}

#[test]
fn test() {
    let mut root = TreeNode::new(3);
    let mut root_right = TreeNode::new(4);
    root_right.left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    root_right.right = Some(Rc::new(RefCell::new(TreeNode::new(2))));

    root.left = Some(Rc::new(RefCell::new(root_right)));
    root.right = Some(Rc::new(RefCell::new(TreeNode::new(5))));


    let mut sub_root = TreeNode::new(4);
    sub_root.left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    sub_root.right = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    let result = Solution::is_subtree(Some(Rc::new(RefCell::new(root))), Some(Rc::new(RefCell::new(sub_root))));
    println!("{}", result);
}