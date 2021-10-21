use std::cell::RefCell;
use std::rc::Rc;
use crate::TreeNode;

struct FindElements {
    nodes: Vec<i32>,
}

impl FindElements {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut nodes = vec![];
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, val: i32, nodes: &mut Vec<i32>) {
            if let Some(node) = root.as_ref() {
                nodes.push(val);
                if node.borrow().left.is_some() {
                    dfs(node.borrow_mut().left.take(), val * 2 + 1, nodes);
                }
                if node.borrow().right.is_some() {
                    dfs(node.borrow_mut().right.take(), val * 2 + 2, nodes);
                }
            }
        }
        dfs(root, 0, &mut nodes);
        Self { nodes }
    }

    fn find(&self, target: i32) -> bool {
        self.nodes.contains(&target)
    }
}

#[test]
fn t() {
    let mut root = TreeNode::new(-1);
    let mut root_right = TreeNode::new(-1);
    root.right = Some(Rc::new(RefCell::new(root_right)));

    let root = FindElements::new(Some(Rc::new(RefCell::new(root))));
    let ret = root.find(2);
    println!("{}", ret);
}









