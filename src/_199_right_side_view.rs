use std::rc::Rc;
use std::cell::RefCell;
use crate::TreeNode;

struct Solution {}

impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn order(root: Option<Rc<RefCell<TreeNode>>>, deep: usize, ans: &mut Vec<i32>) {
            if let Some(node) = root.as_ref() {
                if deep == ans.len() {
                    ans.push(node.borrow().val);
                }

                order(node.borrow_mut().right.take(), deep + 1, ans);
                order(node.borrow_mut().left.take(), deep + 1, ans);
            }
        }

        if root.is_none() {
            return vec![];
        }

        let mut ret = vec![];
        order(root, 0, &mut ret);
        ret
    }
}

#[test]
fn test() {
    let mut root = TreeNode::new(1);
    root.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    let result = Solution::right_side_view(Some(Rc::new(RefCell::new(root))));
    println!("{:?}", result);
}