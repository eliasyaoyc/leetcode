use std::rc::Rc;
use std::cell::RefCell;
use crate::TreeNode;

struct Solution {}

impl Solution {
    pub fn print_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<String>> {
        if root.is_none() {
            return vec![];
        }

        fn highest(root: &Option<Rc<RefCell<TreeNode>>>) -> usize {
            if let Some(node) = root.as_ref() {
                let left = highest(&node.borrow().left);
                let right = highest(&node.borrow().right);
                return left.max(right) + 1;
            }
            return 0;
        }

        let high = highest(&root);

        let mut ret = Vec::with_capacity(high);
        for _ in 0..high {
            let mut t = vec![];
            for _ in 0..(1 << high) - 1 {
                t.push("".to_string());
            }
            ret.push(t);
        }

        fn fill(root: Option<Rc<RefCell<TreeNode>>>, ret: &mut Vec<Vec<String>>, level: usize, l: usize, r: usize) {
            if let Some(node) = root.as_ref() {
                ret[level][(l + r) / 2] = format!("{}", node.borrow().val);
                fill(node.borrow_mut().left.take(), ret, level + 1, l, (l + r) / 2);
                fill(node.borrow_mut().right.take(), ret, level + 1, (l + r) / 2 + 1, r);
            }
        }

        fill(root, &mut ret, 0, 0, (1 << high) - 1);
        ret
    }
}

#[test]
fn t() {
    let mut root = TreeNode::new(1);
    let mut root_right = TreeNode::new(2);
    // root_right.left = Some(Rc::new(RefCell::new(TreeNode::new(4))));

    root.left = Some(Rc::new(RefCell::new(root_right)));
    // root.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));

    let result = Solution::print_tree(Some(Rc::new(RefCell::new(root))));
    println!("{:?}",result)
}

