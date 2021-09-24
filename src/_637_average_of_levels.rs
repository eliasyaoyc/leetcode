use std::rc::Rc;
use std::cell::RefCell;
use crate::TreeNode;

struct Solution {}

impl Solution {
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        fn bfs(nodes: Vec<Rc<RefCell<TreeNode>>>, ret: &mut Vec<f64>) {
            if nodes.is_empty() {
                return;
            }
            let sum = nodes.iter().map(|node| node.borrow().val as i64).sum::<i64>();
            ret.push(sum as f64 / nodes.len() as f64);
            bfs(nodes.iter().fold(vec![], |mut acc, i| {
                if let Some(ref left) = i.borrow().left {
                    acc.push(left.clone());
                }

                if let Some(ref right) = i.borrow().right {
                    acc.push(right.clone());
                }
                acc
            }), ret);
        }
        let mut ret = vec![];
        bfs(vec![root.unwrap()], &mut ret);
        ret
    }
}

#[test]
fn test(){
    println!("{}",15 as f64 / 2 as f64);
}