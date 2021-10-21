use std::rc::Rc;
use std::cell::RefCell;
use crate::TreeNode;

struct Solution {}

impl Solution {
    pub fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut ans = 1;
        let mut que = vec![(0, root)];
        while !que.is_empty() {
            ans = ans.max(que.last().unwrap().0 - que[0].0 + 1);
            let mut tmp = vec![];
            for (i, r) in que {
                let r = r.as_ref().unwrap().borrow();
                if r.left.is_some() {
                    tmp.push((i * 2, r.left.clone()));
                }
                if r.right.is_some() {
                    tmp.push((i * 2 + 1, r.right.clone()));
                }
            }
            que = tmp;
        }
        ans
    }
}

#[test]
fn t() {
    let mut root = TreeNode::new(1);
    let mut root_right = TreeNode::new(2);
    root_right.left = Some(Rc::new(RefCell::new(TreeNode::new(4))));

    root.left = Some(Rc::new(RefCell::new(root_right)));
    root.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));

    let result = Solution::width_of_binary_tree(Some(Rc::new(RefCell::new(root))));
}
