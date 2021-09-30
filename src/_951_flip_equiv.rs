use std::rc::Rc;
use std::cell::RefCell;
use crate::TreeNode;

struct Solution {}

impl Solution {
    pub fn flip_equiv(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn flip(root1: &Option<Rc<RefCell<TreeNode>>>, root2: &Option<Rc<RefCell<TreeNode>>>) -> bool {
            if let (Some(node1), Some(node2)) = (root1, root2) {
                let val1 = node1.borrow().val;
                let val2 = node2.borrow().val;
                let left1 = &node1.borrow().left;
                let right1 = &node1.borrow().right;

                let left2 = &node2.borrow().left;
                let right2 = &node2.borrow().right;

                if val1 != val2 {
                    return false;
                }
                (flip(left1, left2) && flip(right1, right2)) ||
                    (flip(left1, right2) && flip(right1, left2))
            } else {
                root1 == root2
            }
        }
        flip(&root1, &root2)
    }
}

#[test]
fn t() {
    let mut root = TreeNode::new(1);
    let mut root_left = TreeNode::new(2);
    let mut root_right = TreeNode::new(3);

    let mut root_right_right = TreeNode::new(6);


    root_left.left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    root_left.right = Some(Rc::new(RefCell::new(TreeNode::new(5))));


    root_right_right.left = Some(Rc::new(RefCell::new(TreeNode::new(7))));
    root_right_right.right = Some(Rc::new(RefCell::new(TreeNode::new(8))));

    root_right.left = Some(Rc::new(RefCell::new(root_right_right)));

    root.left = Some(Rc::new(RefCell::new(root_left)));
    root.right = Some(Rc::new(RefCell::new(root_right)));


    let mut root1 = TreeNode::new(99);
    let mut root1_left = TreeNode::new(3);
    let mut root1_right = TreeNode::new(2);

    let mut root1_right_right = TreeNode::new(5);


    root1_left.right = Some(Rc::new(RefCell::new(TreeNode::new(6))));


    root1_right_right.left = Some(Rc::new(RefCell::new(TreeNode::new(8))));
    root1_right_right.right = Some(Rc::new(RefCell::new(TreeNode::new(7))));

    root1_right.left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    root1_right.right = Some(Rc::new(RefCell::new(root1_right_right)));

    root1.left = Some(Rc::new(RefCell::new(root1_left)));
    root1.right = Some(Rc::new(RefCell::new(root1_right)));

    println!("{:?}", root);
    println!("{:?}", root1);
    let result = Solution::flip_equiv(Some(Rc::new(RefCell::new(root))), Some(Rc::new(RefCell::new(root1))));
    println!("{}", result);
}