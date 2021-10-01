use std::rc::Rc;
use std::cell::RefCell;
use crate::TreeNode;

struct Solution {}

impl Solution {
    pub fn flip_match_voyage(root: Option<Rc<RefCell<TreeNode>>>, voyage: Vec<i32>) -> Vec<i32> {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, ans: &mut Vec<i32>, index: &mut usize, voyage: Vec<i32>) -> bool {
            if let Some(node) = root.as_ref() {
                if voyage[*index] != node.borrow().val {
                    return false;
                }
                *index += 1;
                if node.borrow().left.is_none() && node.borrow().right.is_none() {
                    return true;
                }
                if node.borrow().left.is_some() {
                    let left = node.borrow_mut().left.take().unwrap();
                    if left.borrow().val == voyage[*index] {
                        dfs(Some(left), ans, index, voyage.clone())
                            && dfs(node.borrow_mut().right.take(), ans, index, voyage.clone())
                    } else {
                        ans.push(node.borrow().val);
                        dfs(node.borrow_mut().right.take(), ans, index, voyage.clone())
                            && dfs(Some(left), ans, index, voyage.clone())
                    }
                } else {
                    dfs(node.borrow_mut().right.take(), ans, index, voyage)
                }
            } else {
                true
            }
        }
        let mut ans = vec![];
        let result = dfs(root, &mut ans, &mut 0_usize, voyage);
        if result {
            ans
        } else {
            vec![-1]
        }
    }
}