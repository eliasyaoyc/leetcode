use std::cell::RefCell;
use std::rc::Rc;
use crate::TreeNode;

struct Codec {}

impl Codec {
    fn new() -> Self {
        Self {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        fn in_order(root: Option<Rc<RefCell<TreeNode>>>, ans: &mut String) {
            if let Some(node) = root.as_ref() {
                ans.push_str(&node.borrow().val.to_string());
                ans.push_str(",");
                in_order(node.borrow_mut().left.take(), ans);
                in_order(node.borrow_mut().right.take(), ans);
            }
        }

        if root.is_none() {
            return String::new();
        }

        let mut ret = String::new();
        in_order(root, &mut ret);
        ret
    }

    //  213   123
    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        if data.is_empty() {
            return None;
        }

        let pre_order: Vec<i32> = data.split(",").filter(|s| s.len() > 0).map(|v| v.parse::<i32>().unwrap()).collect::<Vec<i32>>();

        let mut in_order: Vec<i32> = pre_order.clone();
        in_order.sort();

        fn de_order(pre_order: &[i32], in_order: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if pre_order.is_empty() {
                return None;
            }
            let l = in_order.len();
            let val = pre_order[0];
            let index = in_order.iter().position(|&x| x == val).unwrap();
            let mut root = TreeNode::new(val);

            if index > 0 {
                root.left = de_order(&pre_order[1..index + 1], &in_order[0..index]);
            }
            if index < l - 1 {
                root.right = de_order(&pre_order[index + 1..].to_vec(), &in_order[index + 1..])
            }
            Some(Rc::new(RefCell::new(root)))
        }
        de_order(&pre_order, &in_order)
    }
}

#[test]
fn test() {
    let mut root = TreeNode::new(2);
    root.left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    root.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    let codec = Codec::new();
    let result = codec.serialize(Some(Rc::new(RefCell::new(root))));
    println!("{}",result);
    let ret = codec.deserialize(result);
    print!("{:?}",ret);
}























