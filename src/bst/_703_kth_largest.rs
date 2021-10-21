use std::rc::Rc;
use std::cell::RefCell;
use crate::TreeNode;

struct KthLargest {
    pos: i32,
    nums: Vec<i32>,
}

impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        Self { pos: k, nums }
    }

    fn add(&mut self, val: i32) -> i32 {
        self.nums.push(val);
        self.nums.sort_by(|a, b| b.cmp(a));
        self.nums[self.pos as usize - 1]
    }

    // TODO remove vec.sort
    fn add02(&mut self,val:i32) -> i32 {
        0
    }
}