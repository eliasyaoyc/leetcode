use std::rc::Rc;
use std::cell::RefCell;
use crate::TreeNode;

struct Solution {}

impl Solution {
    pub fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, val: i32, depth: i32, parent: i32, ans: &mut Option<(i32, i32)>) {
            if let Some(node) = root.as_ref() {
                if node.borrow().val == val {
                    *ans = Some((depth, parent));
                }
                let v = node.borrow().val;
                dfs(&node.borrow().left, val, depth + 1, v, ans);
                dfs(&node.borrow().right, val, depth + 1, v, ans);
            }
        }
        let mut x_v: Option<(i32, i32)> = None;
        let mut y_v: Option<(i32, i32)> = None;
        dfs(&root, x, 0, 0, &mut x_v);
        dfs(&root, y, 0, 0, &mut y_v);

        if let (Some((x_d, x_p)), Some((y_d, y_p))) = (x_v, y_v) {
            println!("{},{}", x_d, y_d);
            x_d == y_d && x_p != y_p
        } else { false }
    }
}

#[test]
fn t() {
    let mut root = TreeNode::new(1);
    let mut root_left = TreeNode::new(2);
    let mut root_right = TreeNode::new(3);

    // let mut root_right_right = TreeNode::new(6);


    // root_left.left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    root_left.right = Some(Rc::new(RefCell::new(TreeNode::new(4))));

    // root_right.left = Some(Rc::new(RefCell::new(root_right_right)));
    root_right.right = Some(Rc::new(RefCell::new(TreeNode::new(5))));

    // root.left = Some(Rc::new(RefCell::new(root_left)));
    root.left = Some(Rc::new(RefCell::new(root_left)));
    root.right = Some(Rc::new(RefCell::new(root_right)));

    let result = Solution::is_cousins(Some(Rc::new(RefCell::new(root))), 5, 4);
    println!("{}", result);
}