use std::cell::RefCell;
use std::rc::Rc;
use crate::TreeNode;

struct Solution {}

impl Solution {
    pub fn bst_from_preorder(preorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn construct(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            let n = preorder.len();
            if n == 0 {
                return None;
            } else {
                if n == 1 {
                    Some(Rc::new(RefCell::new(TreeNode::new(preorder[0]))))
                } else {
                    let index = inorder.binary_search(&preorder[0]).unwrap();
                    let mut root = TreeNode::new(preorder[0]);
                    root.left = construct(&preorder[1..=index], &inorder[0..index]);
                    root.right = construct(&preorder[index + 1..], &inorder[index + 1..]);
                    Some(Rc::new(RefCell::new(root)))
                }
            }
        }
        let mut inorder = preorder.clone();
        inorder.sort_unstable();
        construct(&preorder, &inorder)
    }
}

#[test]
fn t() {
    let v = vec![8, 5, 1, 7, 10, 12];

    for x in &v[1..=0] {
        println!("{}",x);
    }
}