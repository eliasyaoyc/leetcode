use std::rc::Rc;
use std::cell::RefCell;
use std::fmt::Debug;
use crate::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut stk = vec![root];
        let mut out = vec![];

        while !stk.is_empty() {
            let mut layer = vec![];
            let mut aux = vec![];

            while !stk.is_empty() {
                if let Some(mut node) = stk.pop().unwrap() {
                    layer.push(node.borrow().val);

                    if out.len() % 2 != 0 {
                        aux.push(node.borrow_mut().right.take());
                        aux.push(node.borrow_mut().left.take());
                    } else {
                        aux.push(node.borrow_mut().left.take());
                        aux.push(node.borrow_mut().right.take());
                    }
                }
            }
            stk = aux;
            out.push(layer);
        }
        out.into_iter().filter(|a| !a.is_empty()).collect()
    }

    pub fn zigzag_level_order_02(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if let Some(i) = root {
            fn breath_order(mut ans: Vec<Vec<i32>>,
                            cur: Vec<Rc<RefCell<TreeNode>>>,
                            reverse: bool) -> Vec<Vec<i32>> {
                if cur.is_empty() {
                    ans
                } else {
                    breath_order(
                        {
                            let tmp = cur.iter().map(|i| i.borrow().val);
                            ans.push(if reverse {
                                tmp.rev().collect()
                            } else {
                                tmp.collect()
                            });
                            ans
                        },
                        cur.iter().fold(vec![], |mut acc, i| {
                            if let Some(ref left) = i.borrow().left {
                                acc.push(left.clone())
                            }
                            if let Some(ref right) = i.borrow().right {
                                acc.push(right.clone())
                            }
                            acc
                        }),
                        !reverse,
                    )
                }
            }
            breath_order(vec![], vec![i], false)
        } else {
            vec![]
        }
    }

    pub fn zigzag_level_order_03(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        fn breath_order(mut ans: Vec<Vec<i32>>,
                        nodes: Vec<Rc<RefCell<TreeNode>>>,
                        order: bool) -> Vec<Vec<i32>> {
            if nodes.is_empty() {
                return ans;
            }
            let tmp = nodes.iter().map(|node| node.borrow().val);
            ans.push(if order {
                tmp.rev().collect()
            } else {
                tmp.collect()
            });

            let cur = nodes.iter().fold(vec![], |mut acc, i| {
                if let Some(ref left) = i.borrow().left {
                    acc.push(left.clone())
                }
                if let Some(ref right) = i.borrow().right {
                    acc.push(right.clone())
                }
                acc
            });

            breath_order(ans, cur, !order)
        }

        if root.is_none() {
            return vec![];
        }

        breath_order(vec![], vec![root.unwrap()], false)
    }
}

#[test]
fn test() {
    println!("{}", 1);
}