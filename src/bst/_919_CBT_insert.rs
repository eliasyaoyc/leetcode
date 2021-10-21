use std::cell::RefCell;
use std::rc::Rc;
use crate::TreeNode;

struct CBTInserter {
    node: Vec<Option<Rc<RefCell<TreeNode>>>>,
}

impl CBTInserter {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut p = vec![root];
        let mut i = 0;
        while i < p.len() {
            let left = p[i].as_ref().unwrap().borrow().left.clone();
            if left.is_some() {
                p.push(left);
            }

            let right = p[i].as_ref().unwrap().borrow().right.clone();
            if right.is_some() {
                p.push(right);
            }
            i += 1;
        }
        Self { node: p }
    }

    fn insert(&mut self, val: i32) -> i32 {
        let n = self.node.len();
        self.node.push(Some(Rc::new(RefCell::new(TreeNode::new(val)))));
        let mut father = self.node[(n - 1) >> 1].as_ref().unwrap().borrow_mut();
        if n & 1 == 1 {
            father.left = self.node[n].clone();
        } else {
            father.right = self.node[n].clone();
        }
        father.val
    }

    fn get_root(&self) -> Option<Rc<RefCell<TreeNode>>> {
        self.node[0].clone()
    }
}

#[test]
fn t() {
    let mut root = TreeNode::new(1);
    let mut root_right = TreeNode::new(2);
    // root_right.left = Some(Rc::new(RefCell::new(TreeNode::new(0))));
    // root_right.right = Some(Rc::new(RefCell::new(TreeNode::new(1))));

    root.left = Some(Rc::new(RefCell::new(root_right)));

    let mut tree = CBTInserter::new(Some(Rc::new(RefCell::new(root))));
    let result = tree.insert(3);
    println!("{}", result);
    let result2 = tree.insert(4);
    println!("{}",result2);
    let root = tree.get_root();
    println!("{:?}", root);
}